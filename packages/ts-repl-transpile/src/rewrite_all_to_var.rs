use swc_core::common::util::take::Take;
use swc_core::ecma::ast::{Expr, KeyValueProp, ModuleItem, Prop, PropOrSpread, Stmt, VarDecl, VarDeclKind};
use swc_core::ecma::utils::quote_ident;
use swc_core::ecma::visit::VisitMut;
use swc_core::ecma::visit::VisitMutWith;

pub struct TransformAllToVar;

impl VisitMut for TransformAllToVar {
    fn visit_mut_var_decl(&mut self, n: &mut VarDecl) {
        n.kind = VarDeclKind::Var;
    }
    // fn visit_mut_object_lit(&mut self, n: &mut ObjectLit) {
    //     n.props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
    //         key: quote_ident!("configurable").into(),
    //         value: Box::new(true.into()),
    //     }))));
    // }
    fn visit_mut_module_items(&mut self, n: &mut Vec<ModuleItem>) {
        n.visit_mut_children_with(self);
        n.iter_mut().for_each(|x| match x {
            ModuleItem::Stmt(x) => {
                match x {
                    Stmt::Expr(y) =>
                        match y.expr.as_mut() {
                            Expr::Call(call_expr) => {
                                if let Some(expr) = call_expr.callee.as_mut_expr() {
                                    match expr.as_mut() {
                                        Expr::Member(x) => {
                                            if let Some(obj_ident) = x.obj.as_mut_ident() {
                                                if obj_ident.sym.to_string() == "Object" {
                                                    x.prop.as_mut_ident().map(|prop_ident| {
                                                        if prop_ident.sym.to_string() == "defineProperty" {
                                                            call_expr.args.iter_mut().for_each(|ent| {
                                                                match ent.expr.as_mut() {
                                                                    Expr::Object(n) => {
                                                                        n.props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                                                                            key: quote_ident!("configurable").into(),
                                                                            value: Box::new(true.into()),
                                                                        }))));
                                                                    }
                                                                    _ => {}
                                                                };
                                                            });
                                                        }
                                                    });
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        },
                    // Stmt::Decl(y) =>
                    //     match y.as_mut_fn_decl() {
                    //         Some(decl) => {
                    //             if decl.ident.sym.to_string() == "_export" {
                    //                 decl.visit_mut_with(self);
                    //             }
                    //         }
                    //         None => {}
                    //     }
                    _ => {}
                }
            }
            _ => {}
        });
    }
    // fn visit_mut_module_items(&mut self, n: &mut Vec<ModuleItem>) {
    //     n.iter_mut().for_each(|x| match x {
    //         ModuleItem::Stmt(x) => {
    //             match x {
    //                 Stmt::Expr(y) =>
    //                     match y.expr.as_mut() {
    //                         Expr::Call(z) => {
    //                             if let Some(f) = z.callee.as_mut_expr() {
    //                                 match f.as_mut() {
    //                                     Expr::Member(x) => {
    //                                         if let Some(y) = x.obj.as_mut_ident() {
    //                                             if y.sym.to_string() == "Object" {
    //                                                 x.prop.as_mut_ident().map(|x| {
    //                                                     if x.sym.to_string() == "defineProperty" {
    //                                                         z.args[0].expr.as_mut_ident().map(|o| {
    //                                                             o.sym.to_string() == "exports"
    //                                                             make_it_configurable()
    //                                                         });
    //                                                     } else {
    //                                                         ()
    //                                                     }
    //                                                 });
    //                                             }
    //                                         }
    //                                     }
    //                                     _ => {}
    //                                 }
    //                             }
    //                         }
    //                         _ => {}
    //                     }
    //                 _ => {}
    //             }
    //         }
    //         _ => {}
    //     });
    // }
}
