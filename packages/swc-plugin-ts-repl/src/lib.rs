use std::env;
use swc_core::common::comments::SingleThreadedComments;
use swc_core::common::errors::{ColorConfig, Handler};
use swc_core::common::SourceMap;
use swc_core::common::sync::Lrc;
use swc_core::ecma::ast::*;
use swc_core::ecma::{
    ast::Program,
    transforms::testing::test,
    visit::{as_folder, FoldWith, VisitMut},
};
use swc_core::ecma::parser::lexer::Lexer;
use swc_core::ecma::parser::Syntax;
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_core::ecma::transforms::module::common_js::*;
use swc_core::common::Mark;
use swc_core::ecma::transforms::base::feature::FeatureFlag;
use swc_core::common::chain;

#[derive(Default)]
pub struct TransformImportNamedToDestructuringRequireVisitor{
    pub has_export_star:Option<bool>,
}

impl TransformImportNamedToDestructuringRequireVisitor {
    pub fn handle_import_decl(&mut self, decl: &mut ModuleDecl) -> Option<(ModuleItem, Vec<ModuleItem>)> {
        let n;
        match decl {
            ModuleDecl::Import(import) => n = import,

            _ => return None,
        }

        if n.type_only {
            return None;
        }

        // echo every name that is used after the import as a workaround for
        // (swc ts strip process that removes imports in typescript that are seen as unused
        // in the compilation unit)
        //
        // e.g. import {foo, bar} from 'baz'
        // => const {foo, bar} = require('baz');foo;bar;
        let mut used_names: Vec<Ident> = vec![];

        // we need these things
        let mut props = vec![];
        // from here
        let module_name = &n.src.value.to_string();

        let mut import_default_name: Option<Ident> = None;

        for specifier in n.specifiers.iter() {
            match &specifier {
                ImportSpecifier::Named(ref x) => {
                    if x.is_type_only {
                        return None;
                    }
                    let alias: Option<Ident>;
                    let name: Ident;
                    match &x.imported {
                        None => {
                            alias = None;
                            // this is fine
                            name = x.clone().local
                        }
                        Some(x1) => {
                            // this is fine
                            alias = Some(x.clone().local);
                            match x1 {
                                // this is fine
                                ModuleExportName::Ident(i) => name = i.clone(),
                                ModuleExportName::Str(_) => {
                                    unreachable!()
                                }
                            }
                        }
                    }
                    props.push((name, alias));
                }
                ImportSpecifier::Namespace(s) => {
                    import_default_name = Some(s.local.clone());
                }
                ImportSpecifier::Default(s) => {
                    import_default_name = Some(s.local.clone())
                }
            }
        }


        if import_default_name.is_some() {
            let used = vec![
                ModuleItem::Stmt(Stmt::Expr(ExprStmt {
                    span: Default::default(),
                    expr: Box::new(Expr::Ident(import_default_name.clone().unwrap())),
                }))
            ];
            let dec = VarDecl {
                span: Default::default(),
                kind: VarDeclKind::Const,
                declare: false,
                decls: vec![VarDeclarator {
                    span: Default::default(),
                    definite: false,
                    name: Pat::Ident(BindingIdent {
                        type_ann: None,
                        id: import_default_name.unwrap(),
                    }),
                    init: Some(Box::new(Expr::Call(CallExpr {
                        span: Default::default(),
                        args: vec![ExprOrSpread {
                            spread: None,
                            expr: Box::new(Expr::Lit(Lit::Str(Str {
                                span: Default::default(),
                                raw: None,
                                value: module_name.clone().into(),
                            }))),
                        }],
                        callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
                            "require".into(),
                            Default::default(),
                        )))),
                        type_args: None,
                    }))),
                }],
            };

            return Some(
                (ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(dec)))),
                 used
                )
            );
        }

        let dec = VarDecl {
            span: Default::default(),
            kind: VarDeclKind::Const,
            declare: false,
            decls: vec![VarDeclarator {
                span: Default::default(),
                // true or false?
                definite: false,
                name: Pat::Object(ObjectPat {
                    span: Default::default(),
                    props: props
                        .iter()
                        .map(|x| {
                            let (name, alias) = x;
                            if alias.is_none() {
                                used_names.push(name.clone());
                                ObjectPatProp::Assign(AssignPatProp {
                                    span: Default::default(),
                                    key: name.clone(),
                                    value: None,
                                })
                            } else {
                                used_names.push(alias.clone().unwrap());
                                ObjectPatProp::KeyValue(KeyValuePatProp {
                                    // this is fine
                                    key: PropName::Ident(name.clone()),
                                    value: Box::new(Pat::Ident(BindingIdent {
                                        type_ann: None,
                                        id: alias.clone().unwrap(),
                                    })),
                                })
                            }
                        })
                        .collect(),
                    optional: false,
                    type_ann: None,
                }),
                init: Some(Box::new(Expr::Call(CallExpr {
                    span: Default::default(),
                    args: vec![ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Lit(Lit::Str(Str {
                            span: Default::default(),
                            raw: None,
                            value: module_name.clone().into(),
                        }))),
                    }],
                    callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
                        "require".into(),
                        Default::default(),
                    )))),
                    type_args: None,
                }))),
            }],
        };
        // let usage = "";
        let name_statements: Vec<ModuleItem> = used_names.into_iter().map(|ident| {
            ModuleItem::Stmt(Stmt::Expr(ExprStmt {
                span: Default::default(),
                expr: Box::new(Expr::Ident(ident)),
            }
            ))
        }).collect();

        let rewritten_import = ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(dec))));
        // println!("{:?}", name_statements);
        Some((rewritten_import, name_statements))
    }
}

impl VisitMut for TransformImportNamedToDestructuringRequireVisitor {
    fn visit_mut_module_items(&mut self, n: &mut Vec<ModuleItem>) {
        let mut extra: Vec<ModuleItem> = vec![];
        n.retain_mut(|item| match item {
            ModuleItem::Stmt(_) => true,
            ModuleItem::ModuleDecl(decl) =>
                match decl {
                    // ModuleDecl::ExportDecl(_) => {}
                    // ModuleDecl::ExportNamed(_) => {}
                    // ModuleDecl::ExportDefaultDecl(_) => {}
                    // ModuleDecl::ExportDefaultExpr(_) => {}
                    ModuleDecl::ExportAll(all) => {
                        self.has_export_star=Some(true);
                        return true;
                    }
                    // ModuleDecl::TsImportEquals(_) => {}
                    // ModuleDecl::TsExportAssignment(_) => {}
                    // ModuleDecl::TsNamespaceExport(_) => {}
                    ModuleDecl::Import(_) =>
                        match self.handle_import_decl(decl) {
                            Some(x) => {
                                // Some(mut x) => {
                                extra.extend(x.1);
                                *item = x.0;
                                true
                            }
                            _ => true,
                        },
                    _ => true,
                }
        });
        // wrong, should be after each import, not all
        (*n).extend(extra)
        // n.extend(vec![ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
        //     span: Default::default(),
        //     declare:false,
        //     kind:VarDeclKind::Var,
        //     decls:vec![VarDeclarator{span: Default::default(),
        //     init:None,
        //     definite:true,
        //     name:Pat::Ident(BindingIdent{type_ann:None,id:Ident{
        //         span:Default::default(),
        //         sym: "".into(),
        //         optional:false,
        //
        //
        //     }})}]
        // }))))])
        // n.extend(vec![""]);
    }
}


// pub struct TransformExportStatementsToExportAssignment;
//
// impl VisitMut for TransformExportStatementsToExportAssignment {
//     fn visit_mut_module_items(&mut self, n: &mut Vec<ModuleItem>) {
//         n.retain_mut(|item| match item {
//             ModuleItem::Stmt(stmt) => match stmt {},
//             ModuleItem::ModuleDecl(decl) => match self.handle_export_decl(decl) {
//                 Some(x) => {
//                     *item = x;
//                     true
//                 }
//                 _ => true,
//             },
//         });
//     }
// }

/// An example plugin function with macro support.
/// `plugin_transform` macro interop pointers into deserialized structs, as well
/// as returning ptr back to host.
///
/// It is possible to opt out from macro by writing transform fn manually
/// if plugin need to handle low-level ptr directly via
/// `__transform_plugin_process_impl(
///     ast_ptr: *const u8, ast_ptr_len: i32,
///     unresolved_mark: u32, should_enable_comments_proxy: i32) ->
///     i32 /*  0 for success, fail otherwise.
///             Note this is only for internal pointer interop result,
///             not actual transform result */`
///
/// This requires manual handling of serialization / deserialization from ptrs.
/// Refer swc_plugin_macro to see how does it work internally.
#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(
        TransformImportNamedToDestructuringRequireVisitor{
            ..Default::default()
        },
    ))
}

test!(
    Default::default(),
    |_| as_folder(TransformImportNamedToDestructuringRequireVisitor),
    transform_all,
    r#"
    import {abc as xyz} from 'foobar';
    import * as myns from 'other'
    import foo from 'bar';
    console.log("transform",xyz,foo,myns);
    "#
    .trim(),
    r#"
    const { abc: xyz  } = require("foobar");
    const myns = require("other");
    const foo = require("bar");
    console.log("transform", xyz, foo, myns);
    "#
    .trim()
);

test!(
    Syntax::Typescript(Default::default()),
    |_| as_folder(TransformImportNamedToDestructuringRequireVisitor),
    transform_all_when_not_referenced,
    r#"
    import {abc as xyz} from 'foobar';
    import * as myns from 'other'
    import foo from 'bar';
    "#
    .trim(),
    r#"
    const { abc: xyz  } = require("foobar");
    const myns = require("other");
    const foo = require("bar");
    "#
    .trim()
);

test!(
    Syntax::Typescript(Default::default()),
    |_|
    chain!(
      as_folder(TransformImportNamedToDestructuringRequireVisitor),
      common_js::<SingleThreadedComments>(
            Mark::new(),
            Default::default(),
            FeatureFlag::default(),
            None
        )
    ) ,
    transform_exports,
    r#"
    import {bar} from 'baz';
    export const foo = 42;
    "#
    .trim(),
    r#"
     "use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
Object.defineProperty(exports, "foo", {
    enumerable: true,
    get: function() {
        return foo;
    }
});
    const { bar  } = require("baz");
    const foo = 42;
    "#
    .trim()
);

// #[test]
// fn main() {
//     let cm: Lrc<SourceMap> = Default::default();
//     let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
//
//     // Real usage
//     // let fm = cm
//     //     .load_file(Path::new("test.js"))
//     //     .expect("failed to load test.js");
//
//     let input = env::args()
//         .nth(1)
//         .expect("please provide the path of input typescript file");
//
//
//     let comments = SingleThreadedComments::default();
//
//     let lexer = Lexer::new(
//         Syntax::Typescript(TsConfig {
//             tsx: input.ends_with(".tsx"),
//             ..Default::default()
//         }),
//         Default::default(),
//         StringInput::from(&*fm),
//         Some(&comments),
//     );
//
//     let mut parser = Parser::new_from(lexer);
//
//     for e in parser.take_errors() {
//         e.into_diagnostic(&handler).emit();
//     }
//
//     let module = parser
//         .parse_module()
//         .map_err(|e| e.into_diagnostic(&handler).emit())
//         .expect("failed to parse module.");
//
//     let globals = Globals::default();
//     GLOBALS.set(&globals, || {
//         let unresolved_mark = Mark::new();
//         let top_level_mark = Mark::new();
//
//         // Optionally transforms decorators here before the resolver pass
//         // as it might produce runtime declarations.
//
//         // Conduct identifier scope analysis
//         let module = module.fold_with(&mut resolver(unresolved_mark, top_level_mark, true));
//
//         // Remove typescript types
//         let module = module.fold_with(&mut strip(top_level_mark));
//
//         // Fix up any identifiers with the same name, but different contexts
//         let module = module.fold_with(&mut hygiene());
//
//         // Ensure that we have enough parenthesis.
//         let module = module.fold_with(&mut fixer(Some(&comments)));
//
//         let mut buf = vec![];
//         {
//             let mut emitter = Emitter {
//                 cfg: swc_ecma_codegen::Config {
//                     minify: false,
//                     ..Default::default()
//                 },
//                 cm: cm.clone(),
//                 comments: Some(&comments),
//                 wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
//             };
//
//             emitter.emit_module(&module).unwrap();
//         }
//
//         println!("{}", String::from_utf8(buf).expect("non-utf8?"));
//     })
// }
