#![deny(clippy::all)]

mod evaluable_spans;
mod rewrite_all_to_var;
mod tla;

#[macro_use]
extern crate napi_derive;

use crate::evaluable_spans::{FindNeighbors, Neighbor};
use crate::rewrite_all_to_var::TransformAllToVar;
use crate::tla::transform_top_level_await;
use napi::Status;
use serde::Serialize;
use std::borrow::Borrow;
use std::sync::Arc;
use swc_core::base::Compiler;
use swc_core::common::comments::{NoopComments, SingleThreadedComments};
use swc_core::common::errors::{ColorConfig, Handler};
use swc_core::common::input::SourceFileInput;
use swc_core::common::sync::Lazy;
use swc_core::common::{FileName, FilePathMapping, Globals, SourceMap};
use swc_core::common::{Mark, GLOBALS};
use swc_core::ecma::ast::EsVersion;
use swc_core::ecma::codegen::text_writer::JsWriter;
use swc_core::ecma::codegen::Emitter;
use swc_core::ecma::parser::lexer::Lexer;
use swc_core::ecma::parser::{Parser, Syntax, TsConfig};
use swc_core::ecma::transforms::base::feature::enable_available_feature_from_es_version;
use swc_core::ecma::transforms::base::helpers::HELPERS;
use swc_core::ecma::transforms::module::common_js::Config as CommonJSConfig;
use swc_core::ecma::transforms::module::util::ImportInterop;
use swc_core::ecma::transforms::module::*;
use swc_core::ecma::transforms::typescript::{
  strip_with_config, Config as TSTransformConfig, ImportsNotUsedAsValues,
};
use swc_core::ecma::visit::*;
use swc_plugin_typescript_repl::TransformImportNamedToDestructuringRequireVisitor;

// Copied from swc
static COMPILER: Lazy<Arc<Compiler>> = Lazy::new(|| {
  let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
  Arc::new(Compiler::new(cm))
});

const EXPORT_STAR_HELPER: &str = r#"function _exportStar(from, to) {
    Object.keys(from).forEach(function(k) {
        if (k !== "default" && !Object.prototype.hasOwnProperty.call(to, k)) {
            Object.defineProperty(to, k, {
                enumerable: true,
                configurable: true,
                get: function() {
                    return from[k];
                }
            });
        }
    });
    return from;
}"#;

// Copied from swc
fn get_compiler() -> Arc<Compiler> {
  COMPILER.clone()
}

#[derive(Serialize)]
#[napi_derive::napi(object)]
pub struct TransformOutput {
  pub code: String,
  pub is_async: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub map: Option<String>,
}
// Copied from swc
#[derive(Serialize)]
#[napi_derive::napi(object)]
pub struct TransformOutputRegular {
  pub code: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub map: Option<String>,
}

#[derive(Serialize)]
#[napi_derive::napi(object)]
pub struct EvaluableSpans {
  pub spans: Vec<Neighbor>,
}

#[napi]
pub fn evaluable_spans(source: String, target: u32) -> napi::Result<EvaluableSpans> {
  // let c = get_compiler();
  let cm: Arc<SourceMap> = Default::default();

  let output = GLOBALS.set(&Default::default(), || {
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
    let sf = cm.new_source_file(FileName::Anon, source);
    let comments = SingleThreadedComments::default();

    let lexer = Lexer::new(
      Syntax::Typescript(TsConfig {
        decorators: true,
        ..Default::default()
      }),
      Default::default(),
      SourceFileInput::from(sf.borrow()),
      Some(&comments),
    );

    let mut parser = Parser::new_from(lexer);

    for err in parser.take_errors() {
      err.into_diagnostic(&handler).emit();
    }

    let module_parse = parser
      .parse_typescript_module()
      .map_err(|err| err.into_diagnostic(&handler).emit());

    // todo. could be err, see whether other side is set up to expect that
    if module_parse.is_err() {
      return Ok(EvaluableSpans { spans: vec![] });
    }

    let mut module = module_parse.unwrap();

    let top_level_mark = Mark::new();

    let mut findn = FindNeighbors {
      neighbors: vec![],
      target_loc: target,
    };
    module.visit_mut_with(&mut findn);

    Ok(EvaluableSpans {
      spans: findn.neighbors,
    })
  });
  output
}

/// Performs a transformation on the source string such that its output
/// is suitable for usage in a REPL environment.
#[napi]
pub fn transform_sync(source: String) -> napi::Result<TransformOutput> {
  // let c = get_compiler();
  // let cm = c.cm.clone();
  let cm: Arc<SourceMap> = Default::default();
  let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
  let sf = cm.new_source_file(FileName::Anon, source);
  let comments = SingleThreadedComments::default();
  let lexer = Lexer::new(
    Syntax::Typescript(TsConfig {
      decorators: true,
      ..Default::default()
    }),
    Default::default(),
    SourceFileInput::from(sf.borrow()),
    Some(&comments),
  );

  let mut parser = Parser::new_from(lexer);

  for err in parser.take_errors() {
    err.into_diagnostic(&handler).emit();
  }

  let globals = Globals::default();
  let output = GLOBALS.set(&globals, || {
    HELPERS.set(&Default::default(), || {
      let top_level_mark = Mark::new();
      let mut module = parser
        .parse_typescript_module()
        .map_err(|err| err.into_diagnostic(&handler).emit())
        .expect("failed to parse module");

      // todo. marks, resolver

      let mut imp_exp_visit = TransformImportNamedToDestructuringRequireVisitor {
        ..Default::default()
      };
      module.visit_mut_with(&mut imp_exp_visit);
      module = module.fold_with(&mut strip_with_config(
        TSTransformConfig {
          import_not_used_as_values: ImportsNotUsedAsValues::Preserve,
          ..Default::default()
        },
        top_level_mark,
      ));

      // HELPERS.set(&Default::default(), || {
      module.visit_mut_with(&mut common_js(
        Mark::new(),
        CommonJSConfig {
          strict: false,
          strict_mode: false,
          // import_interop: Some(ImportInterop::None),
          ..Default::default()
        },
        enable_available_feature_from_es_version(EsVersion::Es2016),
        Some(&comments),
      ));
      // });

      module.visit_mut_with(&mut TransformAllToVar);
      let tla = transform_top_level_await(&module);
      if tla.has_top_level_await {
        module = tla.module.unwrap();
      };

      let mut buf = vec![];
      let mut emitter = Emitter {
        cfg: Default::default(),
        cm: cm.clone(),
        comments: Some(&comments),
        wr: JsWriter::new(cm.clone(), "\n".into(), &mut buf, None),
      };


      emitter
        .emit_module(&module)
        .map(|_| {
          let mut cde = String::from_utf8_lossy(&buf);
          if imp_exp_visit.has_export_star.is_some() {
            cde += "\n";
            cde += EXPORT_STAR_HELPER;
          }
          TransformOutput {
            code: cde.into(),
            is_async: tla.has_top_level_await,
            map: None,
          }
        })
        .map_err(|err| napi::Error::new(Status::GenericFailure, format!("{:?}", err)))
    })
  });
  output
}

/// Translates TS to JS
#[napi]
pub fn transform_sync_regular(source: String) -> napi::Result<TransformOutputRegular> {
  let cm: Arc<SourceMap> = Default::default();
  let globals = Globals::new();

  let output = GLOBALS.set(&globals, || {
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
    let sf = cm.new_source_file(FileName::Anon, source);
    let comments = SingleThreadedComments::default();
    let lexer = Lexer::new(
      Syntax::Typescript(TsConfig {
        decorators: true,
        ..Default::default()
      }),
      Default::default(),
      SourceFileInput::from(sf.borrow()),
      Some(&comments),
    );

    let mut parser = Parser::new_from(lexer);

    for err in parser.take_errors() {
      err.into_diagnostic(&handler).emit();
    }

    let mut module = parser
      .parse_typescript_module()
      .map_err(|err| err.into_diagnostic(&handler).emit())
      .expect("failed to parse module");

    let top_level_mark = Mark::new();
    module = module.fold_with(&mut strip_with_config(
      TSTransformConfig {
        import_not_used_as_values: ImportsNotUsedAsValues::Preserve,
        ..Default::default()
      },
      top_level_mark,
    ));

    module.visit_mut_with(&mut common_js(
      Mark::new(),
      CommonJSConfig {
        strict: false,
        strict_mode: false,
        ..Default::default()
      },
      enable_available_feature_from_es_version(EsVersion::Es2016),
      Some(&comments),
    ));

    // let tla = transform_top_level_await(&module);
    // if tla.has_top_level_await {
    //     module = tla.module.unwrap();
    // };

    let mut buf = vec![];
    let mut emitter = Emitter {
      cfg: Default::default(),
      cm: cm.clone(),
      comments: Some(&comments),
      wr: JsWriter::new(cm.clone(), "\n".into(), &mut buf, None),
    };

    emitter
      .emit_module(&module)
      .map(|_| TransformOutputRegular {
        code: String::from_utf8_lossy(&buf).into(),
        map: None,
      })
      .map_err(|err| napi::Error::new(Status::GenericFailure, format!("{:?}", err)))
  });
  output
}

// can't run tests without a linker error?
// #[test]
// fn test_transform() {
//     assert_eq!(
//         transform_sync(
//             "import * as foo from 'bar'".to_string()
//         ).unwrap().code,
//         "hello"
//     )
// }
