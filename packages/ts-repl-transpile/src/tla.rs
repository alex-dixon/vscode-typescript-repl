use swc_core::base::atoms::JsWord;
use swc_core::common::DUMMY_SP;
use swc_core::common::util::take::Take;
use swc_core::ecma::ast::{ArrowExpr, AssignExpr, BindingIdent, BlockStmt, BlockStmtOrExpr, CallExpr, Decl, Expr, ExprStmt, FnExpr, Function, Ident, KeyValueProp, Module, ModuleItem, ObjectPatProp, op, ParenExpr, Pat, Prop, PropOrSpread, ReturnStmt, Stmt, VarDecl, VarDeclarator, VarDeclKind};
use swc_core::ecma::utils::{contains_top_level_await, ExprFactory, quote_ident};
use swc_core::ecma::visit::Visit;
use swc_core::ecma::visit::VisitWith;
use swc_core::ecma::visit::VisitMut;
use swc_core::ecma::visit::VisitMutWith;

// Does the following:
// 1. Detects whether there is a top level await in the provided source code
// 2. if not, returns "has_top_level_await" false and the ast is unmodified
// 3. if yes, returns "has_top_level_await" true and rewrites the ast as follows:
//    1. any variables declared at the top level of the source code are hoisted to honor the fact they are declaring something
//       in what is effectively their namespace
//    2. All code is wrapped in an async iffee that has been rewritten to bind all its declarations (at its top level)
//       to the hoisted vars from 3.1. This should include all declarable forms. E.g.:
//       var a, b, c, d, e, f, g;
//       const {a, b} = ... anything on RHS ->
//       var a, b;
//      var {a, b} = <whatever>
//      this has the effect of delcaring a,b etc as undefined, then overwriting their value.
//      All repl code is presumed rewritten to var anyway, at least in cases in which it is reachable
//      by subsequent evaluations (may be redefined)
//    3: Further, the resulting ast will have code of the form {value: <async iffee>} (not implemented at this time)
//    4. there is a further case where the user reasonably expects a value to be returned from something like `await myfunc()`
//       but wrapping in an iife and returning the result of that evaluation results in undefined.. So.
//       all of this greatly hides what's really going on but is convenient for a human sitting at a computer/repl.
//       Therefore we derive the following rules:
//       1. the last statement, if it is made with an async/await expression,

// NB:
// certain statements like imports and exports must be at the top level (cannot be inside a function)

// This will possibly run every time someone evaluates code. Which is rather unfortunate. But so it goes...
// I blame some of it on the notion of top level await.
// This rewrites a lot of stuff. It must be fast. Changing to visit mut or just rethinking what's going on
// once initially working is something I really want to do.

pub struct TransformTopLevelAwait {
    pub has_top_level_await: bool,
    pub module: Option<Module>,
    top_level_syms: Vec<JsWord>,
}

impl TransformTopLevelAwait {
    fn new() -> Self {
        Self {
            has_top_level_await: false,
            module: None,
            top_level_syms: vec![],
        }
    }
}

pub fn transform_top_level_await(ast: &Module) -> TransformTopLevelAwait {
    let mut t = TransformTopLevelAwait::new();
    ast.visit_with(&mut t);
    t
}

impl Visit for TransformTopLevelAwait {
    fn visit_module(&mut self, n: &Module) {
        if !contains_top_level_await(n) {
            self.has_top_level_await = false;
        } else {
            self.has_top_level_await = true;
            n.body.visit_with(self);
        }
    }

    fn visit_pat(&mut self, n: &Pat) {
        match n {
            Pat::Ident(x) => {
                self.top_level_syms.push(x.sym.clone())
            }
            Pat::Array(x) => {
                x.elems.iter().for_each(|x| {
                    if x.is_some() {
                        x.as_ref().unwrap().visit_with(self);
                    }
                })
            }
            Pat::Rest(x) => {
                x.arg.as_ref().clone().ident().map(|x| {
                    self.top_level_syms.push(x.sym.clone())
                });
            }
            Pat::Object(x) => {
                x.props.iter().for_each(|x| {
                    match x {
                        ObjectPatProp::KeyValue(x) => {
                            (*x.value).visit_with(self);
                        }
                        ObjectPatProp::Assign(x) => {
                            self.top_level_syms.push(x.key.sym.clone());
                        }
                        ObjectPatProp::Rest(x) => {
                            x.arg.visit_with(self);
                        }
                    }
                })
            }
            Pat::Assign(x) => {
                x.left.visit_with(self);
            }
            Pat::Invalid(_) => {}
            Pat::Expr(_) => {}
        }
    }

    fn visit_module_items(&mut self, n: &[ModuleItem]) {
        let mut async_iife = vec![];
        let mut top_level_passthrough = vec![];

        n.iter().for_each(|x| match x {
            ModuleItem::Stmt(x) => {
                let mut rewritten_stmt:Option<Stmt> = None;
                match x {
                    Stmt::Decl(y) => {
                        match y {
                            Decl::Class(x) => { self.top_level_syms.push(x.ident.sym.clone()); }
                            Decl::Fn(x) => { self.top_level_syms.push(x.ident.sym.clone()); }
                            Decl::Var(x) => {
                                x.decls.iter().for_each(|decl| {
                                    decl.name.visit_with(self);
                                    // all of these things need to be defined outside the iffe
                                    // and all of them may (as well) return a promise that the caller/user would like to evaluate to a value
                                    // or an exception
                                    //
                                    // i.e. any top level statement is now an assignment to forwardly declared variables
                                    rewritten_stmt = Some(Stmt::Expr(ExprStmt {
                                        span: DUMMY_SP,
                                        expr: Box::new(Expr::Assign(AssignExpr {
                                            op: op!("="),
                                            span: DUMMY_SP,
                                            left: decl.name.clone().into(),
                                            right: decl.init.as_ref().unwrap().clone(),
                                        })),
                                    }));
                                });
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
                // put the original into the iffe we're building up...
                // can't decide whether visit or visit mut is best here
                if rewritten_stmt.is_some() {
                    async_iife.push(rewritten_stmt.unwrap());
                } else {
                    async_iife.push(x.clone());
                }
            }
            ModuleItem::ModuleDecl(x) => {
                top_level_passthrough.push(ModuleItem::ModuleDecl(x.clone()));
            }
        });


        let mut body = top_level_passthrough;

        if !self.top_level_syms.is_empty() {
            let hoisted = VarDecl {
                span: DUMMY_SP,
                kind: VarDeclKind::Var,
                declare: false,
                decls: self.top_level_syms.iter().map(|x| {
                    VarDeclarator {
                        span: DUMMY_SP,
                        name: Pat::Ident(BindingIdent {
                            id: Ident::new(x.clone(), DUMMY_SP),
                            type_ann: None,
                        }),
                        init: None,
                        definite: false,
                    }
                }).collect(),
            };
            body.push(ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(hoisted)))));
        }

        if let Some(last) = async_iife.iter().last().cloned() {
            if contains_top_level_await(&last) {
                match last {
                    Stmt::Return(_)=>{},
                    Stmt::Expr(ref x) => {
                        if x.expr.is_await_expr() {
                            let i = async_iife.len() - 1;
                            async_iife[i] = Stmt::Return(ReturnStmt {
                                span: DUMMY_SP,
                                arg: Some(Box::new(x.expr.as_ref().clone())),
                            });
                        }
                    }
                    _ => {}
                }

            }
        };
        let f = ArrowExpr {
            is_generator: false,
            is_async: true,
            params: Default::default(),
            span: DUMMY_SP,
            body: BlockStmtOrExpr::BlockStmt(BlockStmt {
                span: DUMMY_SP,
                stmts: async_iife,
            }),
            type_params: Default::default(),
            return_type: Default::default(),
        };

        // let iife = ExprStmt {
        //     span: DUMMY_SP,
        //     expr: Box::new(Expr::Paren(ParenExpr {
        //         span: DUMMY_SP,
        //         expr: Box::new(Expr::Call(CallExpr {
        //             span: DUMMY_SP,
        //             callee: f.as_callee(),
        //             args: Default::default(),
        //             type_args: Default::default(),
        //         }))
        //     }))
        // };
        let iife = ExprStmt {
            span: DUMMY_SP,
            expr: Box::new(Expr::Call(CallExpr {
                span: DUMMY_SP,
                callee: Box::new(
                    Expr::Paren(ParenExpr {
                        span: DUMMY_SP,
                        expr: Box::new(Expr::Arrow(f)),
                    })).as_callee(),
                args: Default::default(),
                type_args: Default::default(),

            })),
        };

        body.push(ModuleItem::Stmt(Stmt::Expr(iife)));

        self.module = Some(Module {
            span: DUMMY_SP,
            shebang: None,
            body,
        });
    }
}
