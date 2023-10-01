use std::borrow::BorrowMut;
use swc_core::common::{BytePos, Span, Spanned};
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_core::ecma::ast::*;
use serde::Serialize;
use log::{debug};

#[derive(Debug, Serialize)]
#[napi_derive::napi(object)]
pub struct Neighbor {
    pub start: u32,
    pub end: u32,
    pub r#type: &'static str,
}

pub struct FindNeighbors {
    pub target_loc: u32,
    pub neighbors: Vec<Neighbor>,
}


impl From<&mut ModuleDecl> for Neighbor {
    fn from(value: &mut ModuleDecl) -> Self {
        Neighbor {
            start: value.span_lo().0,
            end: value.span_hi().0,
            r#type: "ModuleDecl",
        }
    }
}

macro_rules! impl_from {
    ($typ:ty, $name:expr) => {
        impl From<&mut $typ> for Neighbor {
            fn from(value: &mut $typ) -> Self {
                let n = Neighbor {
                    start: value.span_lo().0,
                    end: value.span_hi().0,
                    r#type: $name,
                };
                debug!("heyo {:?}", &n);
                n
            }
        }

    };
}
macro_rules! impl_from_boxed {
    ($typ:ty, $name:expr) => {
        impl From<&mut Box<$typ>> for Neighbor {
            fn from(value: &mut Box<$typ>) -> Self {
                let n = Neighbor {
                    start: value.span_lo().0,
                    end: value.span_hi().0,
                    r#type: $name,
                };
                debug!("heyo {:?}", &n);
                n
            }
        }

    };
}
macro_rules! impl_from_option_boxed {
    ($typ:ty, $name:expr) => {
        impl From<&mut Option<Box<$typ>>> for Neighbor {
            fn from(opt: &mut Option<Box<$typ>>) -> Self {
                match opt {
                    None => {},
                    Some(value) => {
                        let n = Neighbor {
                            start: value.span_lo().0,
                            end: value.span_hi().0,
                            r#type: $name,
                          };
                        // println!("heyo {:?}", &n);
                        n
                    }
                }
            }
        }
    };
}

// structs with spans that are evaluable
impl_from!(Stmt, "Statement");
impl_from!(ReturnStmt, "ReturnStatement");
impl_from!(BlockStmt, "BlockStatement");
impl_from!(WithStmt, "WithStatement");
impl_from!(LabeledStmt, "LabeledStatement");
impl_from!(IfStmt, "IfStatement");
impl_from!(SwitchStmt, "IfStatement");
impl_from!(ThrowStmt, "ThrowStatement");
impl_from!(TryStmt, "TryStatement");
impl_from!(WhileStmt, "WhileStatement");
impl_from!(DoWhileStmt, "DoWhileStatement");
impl_from!(ForStmt, "ForStatement");
impl_from!(ForInStmt, "ForInStatement");
impl_from!(ForOfStmt, "ForOfStatement");
impl_from!(AwaitExpr, "AwaitExpression");
impl_from!(ArrayLit, "ArrayLiteral");
impl_from!(ArrowExpr, "ArrowFunctionExpression");
impl_from!(AssignExpr, "AssignmentExpression");
impl_from!(BinExpr, "BinaryExpression");
impl_from!(MemberExpr, "MemberExpression");

impl_from!(SuperPropExpr, "SuperPropExpression");
impl_from!(CondExpr, "ConditionalExpression");
impl_from!(CallExpr, "CallExpression");
impl_from!(NewExpr, "NewExpression");
impl_from!(SeqExpr, "SequenceExpression");
impl_from!(Ident, "Identifier");
impl_from!(ClassExpr, "ClassExpression");
impl_from!(YieldExpr, "YieldExpression");
impl_from!(FnExpr, "FunctionExpression");
impl_from!(Function, "Function");
impl_from!(UpdateExpr, "UpdateExpression");


impl_from!(MetaPropExpr, "MetaProperty");
impl_from!(ParenExpr, "ParenthesisExpression");
impl_from!(OptChainExpr, "OptionalChainingExpression");
impl_from!(SwitchCase, "SwitchCase");

impl_from!(OptCall, "CallExpression");


// literals
impl_from!(ObjectLit, "ObjectExpression");
impl_from!(Str, "StringLiteral");
impl_from!(Bool, "BooleanLiteral");
impl_from!(Null, "NullLiteral");
impl_from!(Number, "NumericLiteral");
impl_from!(BigInt, "BigIntLiteral");
impl_from!(Regex, "RegExpLiteral");
impl_from!(Tpl, "TemplateLiteral");


impl_from!(TaggedTpl, "TaggedTemplateExpression");
impl_from!(ThisExpr, "ThisExpression");
impl_from!(UnaryExpr, "UnaryExpression");
impl_from!(ClassMethod, "ClassMethod");
impl_from!(ClassMember, "ClassMember");
impl_from!(ComputedPropName, "Computed");
impl_from!(ClassProp, "ClassProp");
impl_from!(Constructor, "Constructor");
impl_from!(Decorator, "Decorator");

impl_from!(ExportDecl, "ExportDeclaration");
impl_from!(ExportAll, "ExportAllDeclaration");
impl_from!(ExportDefaultDecl, "ExportDefaultDeclaration");
impl_from!(ExportNamedSpecifier, "ExportSpecifier");
impl_from!(ExportDefaultSpecifier, "ExportDefaultSpecifier");
impl_from!(ExportNamespaceSpecifier, "ExportNamespaceSpecifier");
impl_from!(NamedExport, "ExportNamedDeclaration");

impl_from!(VarDecl, "VariableDeclaration");
// impl_from_option_boxed!(Expr, "VariableDeclaration");
impl_from!(ClassDecl, "ClassDeclaration");
impl_from!(FnDecl, "FunctionDeclaration");
impl_from!(Param,"Parameter");
impl_from!(VarDeclarator, "VariableDeclarator");
impl_from!(BindingIdent, "Identifier");


impl_from!(ArrayPat, "ArrayPattern");
impl_from!(RestPat, "RestElement");
impl_from!(ObjectPat, "ObjectPattern");
impl_from!(AssignPat, "AssignPattern");
impl_from!(TsEnumMember, "TsEnumMember");
impl_from!(TsEnumDecl, "TsEnumDeclaration");
impl_from!(KeyValueProp, "KeyValueProperty");
impl_from!(KeyValuePatProp, "KeyValuePatternProperty");
impl_from!(ExprStmt, "ExpressionStatement");

impl_from!(Import, "Import");
impl_from!(ImportDecl, "Import");
impl_from!(ImportStarAsSpecifier, "ImportNamespaceSpecifier");
impl_from!(ImportNamedSpecifier, "ImportSpecifier");
impl_from!(ImportDefaultSpecifier, "ImportDefaultSpecifier");



impl_from_boxed!(VarDecl, "VariableDeclaration");
impl_from_boxed!(TryStmt, "TryStatement");
impl_from_boxed!(TsEnumDecl, "TsEnumDeclaration");


// impl From<&mut Option<Box<Expr>>> for Neighbor {
//     fn from(opt: &mut Option<Box<Expr>>) -> Self {
//         match opt {
//             None => {}
//             Some(value) => {
//                 match &**value {
//                     Expr::This(x) => { x.into() }
//                     Expr::Array(_) => {}
//                     Expr::Object(_) => {}
//                     Expr::Fn(_) => {}
//                     Expr::Unary(_) => {}
//                     Expr::Update(_) => {}
//                     Expr::Bin(_) => {}
//                     Expr::Assign(_) => {}
//                     Expr::Member(_) => {}
//                     Expr::SuperProp(_) => {}
//                     Expr::Cond(_) => {}
//                     Expr::Call(_) => {}
//                     Expr::New(_) => {}
//                     Expr::Seq(_) => {}
//                     Expr::Ident(_) => {}
//                     Expr::Lit(_) => {}
//                     Expr::Tpl(_) => {}
//                     Expr::TaggedTpl(_) => {}
//                     Expr::Arrow(_) => {}
//                     Expr::Class(_) => {}
//                     Expr::Yield(_) => {}
//                     Expr::MetaProp(_) => {}
//                     Expr::Await(_) => {}
//                     Expr::Paren(_) => {}
//                     Expr::JSXMember(_) => {}
//                     Expr::JSXNamespacedName(_) => {}
//                     Expr::JSXEmpty(_) => {}
//                     Expr::JSXElement(_) => {}
//                     Expr::JSXFragment(_) => {}
//                     Expr::TsTypeAssertion(_) => {}
//                     Expr::TsConstAssertion(_) => {}
//                     Expr::TsNonNull(_) => {}
//                     Expr::TsAs(_) => {}
//                     Expr::TsInstantiation(_) => {}
//                     Expr::TsSatisfies(_) => {}
//                     Expr::PrivateName(_) => {}
//                     Expr::OptChain(_) => {}
//                     Expr::Invalid(_) => {}
//                 }
//                 let n = Neighbor {
//                     start: value.span_lo().0,
//                     end: value.span_hi().0,
//                     r#type: "VariableDeclaration",
//                 };
//                 println!("heyo {:?}", &n);
//                 n
//             }
//         }
//     }
// }

macro_rules! add_and_descend {
    ($self:ident,
        $x:ident,
    [$( $Foo:path ),*]
    ) => {
        match $x {
            $($Foo(a) => {
            // println!("adding and descending {:?}", a.clone());
                if $self.is_in(a.span()) {
                   $self.neighbors.push(a.into());
                   a.visit_mut_children_with($self)
                }
            },)*
            _ => {},
        }
    }
}
// macro_rules! add_enum_members {
//     ($self:ident,
//         $x:ident,
//     [$( $Foo:path ),*]
//     // [$( $Foo:ident ),*]
//     ) => {
//         match $x {
//             $($Foo(a) => {
//                 if $self.is_in(a.span()) {
//                    $self.neighbors.push(a.into())
//                 }
//                 a.visit_mut_with($self)
//             },)*
//             _=>{},
//         }
//     }
// }
// macro_rules! add_and_descend_skip {
//     ($self:ident,
//         $x:ident,
//     [$( $Foo:path ),*]
//     // [$( $Foo:ident ),*]
//     ) => {
//         match $x {
//             $($Foo(a) => { },)*
//             $($Foo(a) => {
//                 if $self.is_in(a.span()) {
//                    $self.neighbors.push(a.into())
//                 }
//                 a.visit_mut_with($self)
//             },)*
//             _=>{},
//         }
//     }
// }
macro_rules! add_if_in_span {
    ($self:ident, $a:ident ) => {
        // println!("adding span, not visiting {:?}", $a);
        if $self.is_in($a.span()) {
           $self.neighbors.push($a.into())
        }
    }
}
macro_rules! add_and_visit_if_in_span {
    ($self:ident, $a:ident ) => {
        // println!("adding and visiting children {:?}", $a);
        if $self.is_in($a.span()) {
           $self.neighbors.push($a.into());
           $a.visit_mut_children_with($self)
        }
    }
}
macro_rules! when_some_visit_all_children_with_self {
    ($self:ident, $a:ident) => {
        match $a {
            None => {},
            Some(x) => x.visit_mut_children_with($self)
        }
    }
}

impl VisitMut for FindNeighbors {
    fn visit_mut_module_item(&mut self, x: &mut ModuleItem) {
        match x {
            ModuleItem::ModuleDecl(d) => {
                if self.is_in(d.span()) {
                    self.neighbors.push(d.into())
                }
                d.visit_mut_children_with(self);
            }
            ModuleItem::Stmt(s) => {
                s.visit_mut_with(self);
            }
        }
    }


    /// enum / union types
    fn visit_mut_expr(&mut self, n: &mut Expr) {
        // println!("visit expr, {}, {}, \n{:?}", n.span_lo().0, n.span_hi().0, serde_json::to_string(n).unwrap());
        match n {
            Expr::Lit(l) => {
                l.visit_mut_with(self);
                return;
            }
            // don't want to visit all children (may have types)
            Expr::Arrow(l) => {
                l.visit_mut_with(self);
                return;
            }
            Expr::Member(l) => {
                // only want to visit the obj prop for this, so don't delegate to add_and_descend
                // which calls visit_mut_children_with
                l.visit_mut_with(self);
                return;
            }

            _ => {}
        };

        // visit these enums and all children
        add_and_descend!(self, n,
            [
            Expr::This,
            Expr::Array,
            Expr::Object,
            Expr::Fn,
            Expr::Unary,
            Expr::Update,
            Expr::Bin,
            Expr::Assign,
            // Expr::Member,
            Expr::SuperProp,
            Expr::Cond,
            Expr::Call,
            Expr::New,
            Expr::Seq,
            Expr::Ident,
            Expr::Tpl,
            Expr::TaggedTpl,
            Expr::Class,
            Expr::Yield,
            Expr::MetaProp,
            Expr::Await,
            Expr::Paren,
            Expr::OptChain

            // is union type, handled above
            // Expr::Lit,

            // visit and only and only children, handled above
            // Expr::Arrow


            // not visiting
            // Expr::PrivateName,
            // Expr::JSXMember
            // Expr::JSXNamespacedName
            // Expr::JSXEmpty
            // Expr::JSXElement
            // Expr::JSXFragment
            // Expr::TsTypeAssertion
            // Expr::TsConstAssertion
            // Expr::TsNonNull
            // Expr::TsAs
            // Expr::TsInstantiation
            // Expr::TsSatisfies
            // Expr::Invalid
        ]);
    }
    fn visit_mut_stmt(&mut self, n: &mut Stmt) {
        match n {
            Stmt::Expr(s) => {
                s.visit_mut_with(self);
            }
            Stmt::Decl(s) => {
                s.visit_mut_with(self);
            }
            _ => {}
        };
        add_and_descend!(self, n, [
                        Stmt::Return,
                        Stmt::Block,
                        Stmt::If,
                        Stmt::Switch,
                        Stmt::Throw,
                        Stmt::While,
                        Stmt::DoWhile,
                        Stmt::For,
                        Stmt::ForIn,
                        Stmt::ForOf,
                        Stmt::Try
                        // enums ^ 2, handled above
                        // Stmt::Decl
                        // Stmt::Expr
                    ]);
    }
    // fn visit_mut_stmts(&mut self, n: &mut Vec<Stmt>) { add_if_in_span!(self, n); }

    fn visit_mut_decl(&mut self, n: &mut Decl) {
        add_and_descend!(
            self, n,
            [
                Decl::Class,
                Decl::Fn,
                Decl::Var,
                Decl::TsEnum
                // not visiting
                // Decl::TsInterface
                // Decl::TsTypeAlias
                // Decl::TsModule
            ]
        );
    }

    // fn visit_mut_default_decl(&mut self, n: &mut DefaultDecl) {
    //     match n {
    //         DefaultDecl::Class(x) => { x.visit_mut_with(self) }
    //         DefaultDecl::Fn(x) => { x.visit_mut_with(self) }
    //         DefaultDecl::TsInterfaceDecl(_) => {}
    //     }
    // }
    //
    // fn visit_mut_script(&mut self, n: &mut Script) { add_if_in_span!(self, n); }
    //
    // // module
    // fn visit_mut_module(&mut self, n: &mut Module) { add_if_in_span!(self, n); }
    // fn visit_mut_module_decl(&mut self, n: &mut ModuleDecl) { add_if_in_span!(self, n); } // enum
    // fn visit_mut_module_export_name(&mut self, n: &mut ModuleExportName) { add_if_in_span!(self, n); }


    // imports
    fn visit_mut_import(&mut self, n: &mut Import) { add_and_visit_if_in_span!(self, n); }
    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) { add_and_visit_if_in_span!(self, n); }
    fn visit_mut_import_default_specifier(&mut self, n: &mut ImportDefaultSpecifier) { add_and_visit_if_in_span!(self, n); }
    fn visit_mut_import_named_specifier(&mut self, n: &mut ImportNamedSpecifier) { add_and_visit_if_in_span!(self, n); }
    fn visit_mut_import_star_as_specifier(&mut self, n: &mut ImportStarAsSpecifier) { add_and_visit_if_in_span!(self, n); }
    // fn visit_mut_import_specifier(&mut self, n: &mut ImportSpecifier) { add_if_in_span!(self, n); }
    // fn visit_mut_import_specifiers(&mut self, n: &mut Vec<ImportSpecifier>) { add_if_in_span!(self, n); }


    // exports
    fn visit_mut_export_all(&mut self, n: &mut ExportAll) { add_and_visit_if_in_span!(self, n); }
    fn visit_mut_export_decl(&mut self, n: &mut ExportDecl) { add_and_visit_if_in_span!(self, n); }
    fn visit_mut_export_default_decl(&mut self, n: &mut ExportDefaultDecl) { add_and_visit_if_in_span!(self, n); }
    fn visit_mut_export_default_specifier(&mut self, n: &mut ExportDefaultSpecifier) { add_if_in_span!(self, n); }
    fn visit_mut_named_export(&mut self, n: &mut NamedExport) { add_if_in_span!(self, n); }
    fn visit_mut_export_named_specifier(&mut self, n: &mut ExportNamedSpecifier) { add_and_visit_if_in_span!(self, n); }
    // fn visit_mut_export_default_expr(&mut self, n: &mut ExportDefaultExpr) { add_if_in_span!(self, n); }
    // fn visit_mut_export_specifier(&mut self, n: &mut ExportSpecifier) { add_if_in_span!(self, n); }
    // fn visit_mut_export_specifiers(&mut self, n: &mut Vec<ExportSpecifier>) { add_if_in_span!(self, n); }


    // not interesting : keywordlike

    // fn visit_mut_continue_stmt(&mut self, n: &mut ContinueStmt) { add_if_in_span!(self, n); }
    // fn visit_mut_debugger_stmt(&mut self, n: &mut DebuggerStmt) { add_if_in_span!(self, n); }
    //

    // fn visit_mut_array_pat(&mut self, n: &mut ArrayPat) { add_if_in_span!(self, n); }
    fn visit_mut_arrow_expr(&mut self, n: &mut ArrowExpr) {
        add_if_in_span!(self, n);
        n.params.visit_mut_with(self);
        // n.params.visit_mut_children_with(self);
        // for mut param in n.params {
        //     param.visit_mut_with(self);
        // }
        n.body.visit_mut_with(self);
        // skip types
        // n.return_type
        // n.type_params
    }
    // fn visit_mut_assign_expr(&mut self, n: &mut AssignExpr) { add_if_in_span!(self, n); }
    // fn visit_mut_assign_op(&mut self, n: &mut AssignOp) { add_if_in_span!(self, n); }
    // fn visit_mut_assign_pat(&mut self, n: &mut AssignPat) { add_if_in_span!(self, n); }
    // fn visit_mut_assign_pat_prop(&mut self, n: &mut AssignPatProp) { add_if_in_span!(self, n); }
    // fn visit_mut_assign_prop(&mut self, n: &mut AssignProp) { add_if_in_span!(self, n); }
    // fn visit_mut_await_expr(&mut self, n: &mut AwaitExpr) { add_if_in_span!(self, n); }
    // // fn visit_mut_big_int_value(&mut self, n: &mut BigIntValue) { add_if_in_span!(self, n); }
    // fn visit_mut_bin_expr(&mut self, n: &mut BinExpr) { add_if_in_span!(self, n); }
    // fn visit_mut_binary_op(&mut self, n: &mut BinaryOp) { add_if_in_span!(self, n); }

    // the ident span range includes the range for type annotations so remove those
    fn visit_mut_binding_ident(&mut self, n: &mut BindingIdent) {
        match &n.type_ann {
            Some(x) => {
                let ident_end = n.type_ann.as_ref().unwrap().span.lo.0 ;
                let ident_start = n.id.span.lo.0;
                // is target in span?
                if self.target_loc >= ident_start && self.target_loc <= ident_end {
                    self.neighbors.push(Ident::new(n.id.sym.clone(), Span {
                        ctxt: Default::default(),
                        lo: BytePos(ident_start),
                        hi: BytePos(ident_end),
                    }).borrow_mut().into())
                }
            }
            None => add_if_in_span!(self, n)
        }
    }
    fn visit_mut_block_stmt(&mut self, n: &mut BlockStmt) { add_and_visit_if_in_span!(self, n); }
    // fn visit_mut_block_stmt_or_expr(&mut self, n: &mut BlockStmtOrExpr) { add_if_in_span!(self, n); }
    // fn visit_mut_break_stmt(&mut self, n: &mut BreakStmt) { add_if_in_span!(self, n); }
    fn visit_mut_call_expr(&mut self, n: &mut CallExpr) { add_and_visit_if_in_span!(self, n); }
    // fn visit_mut_callee(&mut self, n: &mut Callee) { add_if_in_span!(self, n); }
    // fn visit_mut_catch_clause(&mut self, n: &mut CatchClause) { add_if_in_span!(self, n); }
    // fn visit_mut_class(&mut self, n: &mut Class) { add_if_in_span!(self, n); }
    // fn visit_mut_class_decl(&mut self, n: &mut ClassDecl) { add_if_in_span!(self, n); }
    // fn visit_mut_class_expr(&mut self, n: &mut ClassExpr) { add_if_in_span!(self, n); }
    // fn visit_mut_class_member(&mut self, n: &mut ClassMember) { add_if_in_span!(self, n); }
    // fn visit_mut_class_members(&mut self, n: &mut Vec<ClassMember>) { add_if_in_span!(self, n); }
    // fn visit_mut_class_method(&mut self, n: &mut ClassMethod) { add_if_in_span!(self, n); }
    // fn visit_mut_class_prop(&mut self, n: &mut ClassProp) { add_if_in_span!(self, n); }
    // fn visit_mut_computed_prop_name(&mut self, n: &mut ComputedPropName) { add_if_in_span!(self, n); }
    // fn visit_mut_cond_expr(&mut self, n: &mut CondExpr) { add_if_in_span!(self, n); }
    // fn visit_mut_constructor(&mut self, n: &mut Constructor) { add_if_in_span!(self, n); }
    //
    //
    // fn visit_mut_decorator(&mut self, n: &mut Decorator) { add_if_in_span!(self, n); }
    // fn visit_mut_decorators(&mut self, n: &mut Vec<Decorator>) { add_if_in_span!(self, n); }

    // fn visit_mut_empty_stmt(&mut self, n: &mut EmptyStmt) { add_if_in_span!(self, n); }

    /// loops
    fn visit_mut_while_stmt(&mut self, n: &mut WhileStmt) { add_and_visit_if_in_span!(self, n); }
    fn visit_mut_do_while_stmt(&mut self, n: &mut DoWhileStmt) { add_and_visit_if_in_span!(self, n); }
    fn visit_mut_for_in_stmt(&mut self, n: &mut ForInStmt) { add_and_visit_if_in_span!(self, n); }
    fn visit_mut_for_of_stmt(&mut self, n: &mut ForOfStmt) { add_and_visit_if_in_span!(self, n); }
    fn visit_mut_for_stmt(&mut self, n: &mut ForStmt) { add_and_visit_if_in_span!(self, n); }

    /// functions
    fn visit_mut_fn_decl(&mut self, n: &mut FnDecl) { add_and_visit_if_in_span!(self, n); }
    fn visit_mut_fn_expr(&mut self, n: &mut FnExpr) { add_and_visit_if_in_span!(self, n); }
    fn visit_mut_function(&mut self, n: &mut Function) {
        add_if_in_span!(self, n);
        n.params.visit_mut_with(self);
        match &mut n.body {
            Some(x) => { add_and_visit_if_in_span!(self, x); }
            None => {}
        };
        // n.return_type
        // n.type_params
    }


    // fn visit_mut_expr_or_spread(&mut self, n: &mut ExprOrSpread) { add_if_in_span!(self, n); }
    // fn visit_mut_expr_or_spreads(&mut self, n: &mut Vec<ExprOrSpread>) { add_if_in_span!(self, n); }
    fn visit_mut_expr_stmt(&mut self, n: &mut ExprStmt) { add_and_visit_if_in_span!(self, n); }
    // fn visit_mut_exprs(&mut self, n: &mut Vec<Box<Expr>>) { add_if_in_span!(self, n); }


    // this will add all identifiers we navigate to, even if they're in types
    fn visit_mut_ident(&mut self, n: &mut Ident) { add_if_in_span!(self, n); }


    /// control flow
    fn visit_mut_if_stmt(&mut self, n: &mut IfStmt) { add_if_in_span!(self, n); }
    fn visit_mut_switch_case(&mut self, n: &mut SwitchCase) { add_and_visit_if_in_span!(self, n); }
    // fn visit_mut_switch_cases(&mut self, n: &mut Vec<SwitchCase>) { add_if_in_span!(self, n); }
    fn visit_mut_switch_stmt(&mut self, n: &mut SwitchStmt) { add_and_visit_if_in_span!(self, n); }


    fn visit_mut_key_value_pat_prop(&mut self, n: &mut KeyValuePatProp) { add_and_visit_if_in_span!(self, n); }
    fn visit_mut_key_value_prop(&mut self, n: &mut KeyValueProp) { add_and_visit_if_in_span!(self, n); }
    // fn visit_mut_labeled_stmt(&mut self, n: &mut LabeledStmt) { add_if_in_span!(self, n); }


    fn visit_mut_member_expr(&mut self, n: &mut MemberExpr) {
        add_if_in_span!(self, n);
        // prop will return the individual ident divorced of the path,
        // visiting obj means we will return all path segments
        // e.g. a.b.c -> a, a.b, a.b.c
        // but not b, c individually
        n.obj.visit_mut_children_with(self);
        // skipping
        // n.prop
    }
    fn visit_mut_member_prop(&mut self, n: &mut MemberProp) {
        match n {
            MemberProp::Ident(x) => { add_if_in_span!(self, x); }
            MemberProp::PrivateName(_) => {}
            MemberProp::Computed(_) => {}
        }
    }
    // fn visit_mut_meta_prop_expr(&mut self, n: &mut MetaPropExpr) { add_if_in_span!(self, n); }
    // fn visit_mut_meta_prop_kind(&mut self, n: &mut MetaPropKind) { add_if_in_span!(self, n); }
    // fn visit_mut_method_kind(&mut self, n: &mut MethodKind) { add_if_in_span!(self, n); }
    // fn visit_mut_method_prop(&mut self, n: &mut MethodProp) { add_if_in_span!(self, n); }
    //
    //
    // // fn visit_mut_module_items(&mut self, n: &mut Vec<ModuleItem>) { add_if_in_span!(self, n); }
    // fn visit_mut_new_expr(&mut self, n: &mut NewExpr) { add_if_in_span!(self, n); }
    //
    fn visit_mut_object_pat(&mut self, n: &mut ObjectPat) { add_and_visit_if_in_span!(self, n); }
    // fn visit_mut_object_pat_prop(&mut self, n: &mut ObjectPatProp) { add_and_visit_if_in_span!(self, n); }
    // fn visit_mut_object_pat_props(&mut self, n: &mut Vec<ObjectPatProp>) { add_if_in_span!(self, n); }

    /// literals
    fn visit_mut_lit(&mut self, n: &mut Lit) {
        add_and_descend!(self, n, [
            Lit::Str,
            Lit::Bool,
            Lit::Null,
            Lit::Num,
            Lit::BigInt,
            Lit::Regex
           // Lit::JSXText
        ])
    }
    fn visit_mut_null(&mut self, n: &mut Null) { add_if_in_span!(self, n); }
    fn visit_mut_number(&mut self, n: &mut Number) { add_if_in_span!(self, n); }
    fn visit_mut_regex(&mut self, n: &mut Regex) { add_if_in_span!(self, n); }
    fn visit_mut_str(&mut self, n: &mut Str) { add_if_in_span!(self, n); }
    fn visit_mut_bool(&mut self, n: &mut Bool) { add_if_in_span!(self, n); }
    fn visit_mut_big_int(&mut self, n: &mut BigInt) { add_if_in_span!(self, n); }
    fn visit_mut_object_lit(&mut self, n: &mut ObjectLit) { add_and_visit_if_in_span!(self, n); }
    fn visit_mut_array_lit(&mut self, n: &mut ArrayLit) { add_and_visit_if_in_span!(self, n); }


    fn visit_mut_param(&mut self, n: &mut Param) { add_and_visit_if_in_span!(self, n); }
    fn visit_mut_param_or_ts_param_prop(&mut self, n: &mut ParamOrTsParamProp) {
        match n {
            ParamOrTsParamProp::TsParamProp(_) => {}
            ParamOrTsParamProp::Param(x) => { x.visit_mut_with(self); }
        }
    }
    // fn visit_mut_param_or_ts_param_props(&mut self, n: &mut Vec<ParamOrTsParamProp>) { add_if_in_span!(self, n); }
    fn visit_mut_params(&mut self, n: &mut Vec<Param>) {
        // println!("visiting vec of param");
        n.iter_mut().for_each(|param| {
            param.visit_mut_with(self);
        })
    }
    fn visit_mut_paren_expr(&mut self, n: &mut ParenExpr) { add_and_visit_if_in_span!(self, n); }

    fn visit_mut_pat(&mut self, n: &mut Pat) {
        match n {
            Pat::Ident(i) => {
                // force visit binding ident for special handling of span that includes type annotation
                i.visit_mut_with(self)
            }
            _ => {}
            // Pat::Array(_) => {}
            // Pat::Rest(_) => {}
            // Pat::Object(_) => {}
            // Pat::Assign(_) => {}
            // Pat::Invalid(_) => {}
            // Pat::Expr(_) => {}
        }
        add_and_descend!(self, n, [
            // handled above
            // Pat::Ident,

            // TODO. these will probably need special handling if the span of the ident includes the type annotation
            // like Pat::Ident
            Pat::Array,
            Pat::Rest,
            Pat::Object,
            Pat::Assign
            // ignored
            // Pat::Invalid
            // Pat::Expr
        ])
    }
    // fn visit_mut_pat_or_expr(&mut self, n: &mut PatOrExpr) { add_if_in_span!(self, n); }
    // fn visit_mut_pats(&mut self, n: &mut Vec<Pat>) { add_if_in_span!(self, n); }

    // fn visit_mut_program(&mut self, n: &mut Program) { add_if_in_span!(self, n); }

    // union types that should be visited but cannot be added directly. should be "visit all with self"
    // fn visit_mut_prop(&mut self, n: &mut Prop) { add_if_in_span!(self, n); }
    // fn visit_mut_prop_name(&mut self, n: &mut PropName) { add_if_in_span!(self, n); }
    // fn visit_mut_prop_or_spread(&mut self, n: &mut PropOrSpread) { add_if_in_span!(self, n); }
    // fn visit_mut_prop_or_spreads(&mut self, n: &mut Vec<PropOrSpread>) { add_if_in_span!(self, n); }

    // fn visit_mut_reserved_unused(&mut self, n: &mut ReservedUnused) { add_if_in_span!(self, n); }
    // fn visit_mut_rest_pat(&mut self, n: &mut RestPat) { add_if_in_span!(self, n); }

    // fn visit_mut_return_stmt(&mut self, n: &mut ReturnStmt) {
    //     match &n.arg {
    //         None => {}
    //         Some(ref s) => {
    //             match &**s {
    //                 Expr::This(_) => {}
    //                 Expr::Array(_) => {}
    //                 Expr::Object(_) => {}
    //                 Expr::Fn(_) => {}
    //                 Expr::Unary(_) => {}
    //                 Expr::Update(_) => {}
    //                 Expr::Bin(_) => {}
    //                 Expr::Assign(_) => {}
    //                 Expr::Member(_) => {}
    //                 Expr::SuperProp(_) => {}
    //                 Expr::Cond(_) => {}
    //                 Expr::Call(_) => {}
    //                 Expr::New(_) => {}
    //                 Expr::Seq(_) => {}
    //                 Expr::Ident(_) => {}
    //                 Expr::Lit(_) => {}
    //                 Expr::Tpl(_) => {}
    //                 Expr::TaggedTpl(_) => {}
    //                 Expr::Arrow(_) => {}
    //                 Expr::Class(_) => {}
    //                 Expr::Yield(_) => {}
    //                 Expr::MetaProp(_) => {}
    //                 Expr::Await(_) => {}
    //                 Expr::Paren(_) => {}
    //                 Expr::JSXMember(_) => {}
    //                 Expr::JSXNamespacedName(_) => {}
    //                 Expr::JSXEmpty(_) => {}
    //                 Expr::JSXElement(_) => {}
    //                 Expr::JSXFragment(_) => {}
    //                 Expr::TsTypeAssertion(_) => {}
    //                 Expr::TsConstAssertion(_) => {}
    //                 Expr::TsNonNull(_) => {}
    //                 Expr::TsAs(_) => {}
    //                 Expr::TsInstantiation(_) => {}
    //                 Expr::TsSatisfies(_) => {}
    //                 Expr::PrivateName(_) => {}
    //                 Expr::OptChain(_) => {}
    //                 Expr::Invalid(_) => {}
    //             }
    //         }
    //     }
    //     add_if_in_span!(self, n);
    // }

    // fn visit_mut_seq_expr(&mut self, n: &mut SeqExpr) { add_if_in_span!(self, n); }
    // fn visit_mut_spread_element(&mut self, n: &mut SpreadElement) { add_if_in_span!(self, n); }
    // fn visit_mut_static_block(&mut self, n: &mut StaticBlock) { add_if_in_span!(self, n); }


    /// classes / oo
    fn visit_mut_this_expr(&mut self, n: &mut ThisExpr) { add_if_in_span!(self, n); }
    // fn visit_mut_super(&mut self, n: &mut Super) { add_if_in_span!(self, n); }
    // fn visit_mut_super_prop(&mut self, n: &mut SuperProp) { add_if_in_span!(self, n); }
    // fn visit_mut_super_prop_expr(&mut self, n: &mut SuperPropExpr) { add_if_in_span!(self, n); }
    // fn visit_mut_private_method(&mut self, n: &mut PrivateMethod) { add_if_in_span!(self, n); }
    // fn visit_mut_private_name(&mut self, n: &mut PrivateName) { add_if_in_span!(self, n); }
    // fn visit_mut_private_prop(&mut self, n: &mut PrivateProp) { add_if_in_span!(self, n); }
    // fn visit_mut_getter_prop(&mut self, n: &mut GetterProp) { add_if_in_span!(self, n); }
    // fn visit_mut_setter_prop(&mut self, n: &mut SetterProp) { add_if_in_span!(self, n); }
    // fn visit_mut_accessibility(&mut self, n: &mut Accessibility) { add_if_in_span!(self, n); }


    fn visit_mut_throw_stmt(&mut self, n: &mut ThrowStmt) { add_and_visit_if_in_span!(self, n); }

    fn visit_mut_tagged_tpl(&mut self, n: &mut TaggedTpl) { add_and_visit_if_in_span!(self, n); }
    fn visit_mut_tpl(&mut self, n: &mut Tpl) { add_and_visit_if_in_span!(self, n); }
    // fn visit_mut_tpl_element(&mut self, n: &mut TplElement) { add_if_in_span!(self, n); }
    // fn visit_mut_tpl_elements(&mut self, n: &mut Vec<TplElement>) { add_if_in_span!(self, n); }

    fn visit_mut_try_stmt(&mut self, n: &mut TryStmt) { add_and_visit_if_in_span!(self, n); }

    /// ops
    // fn visit_mut_true_plus_minus(&mut self, n: &mut TruePlusMinus) { add_if_in_span!(self, n); }
    // fn visit_mut_unary_expr(&mut self, n: &mut UnaryExpr) { add_if_in_span!(self, n); }
    // fn visit_mut_unary_op(&mut self, n: &mut UnaryOp) { add_if_in_span!(self, n); }
    // fn visit_mut_update_expr(&mut self, n: &mut UpdateExpr) { add_if_in_span!(self, n); }
    // fn visit_mut_update_op(&mut self, n: &mut UpdateOp) { add_if_in_span!(self, n); }

    fn visit_mut_var_decl(&mut self, n: &mut VarDecl) { add_and_visit_if_in_span!(self, n); }
    // fn visit_mut_var_decl_kind(&mut self, n: &mut VarDeclKind) { add_if_in_span!(self, n); }

    fn visit_mut_var_decl_or_expr(&mut self, n: &mut VarDeclOrExpr) {
        match n {
            VarDeclOrExpr::VarDecl(x) => { add_and_visit_if_in_span!(self,x); }
            VarDeclOrExpr::Expr(x) => { x.visit_mut_with(self); }
        }
    }
    // fn visit_mut_var_decl_or_pat(&mut self, n: &mut VarDeclOrPat) { add_if_in_span!(self, n); }
    fn visit_mut_var_declarator(&mut self, n: &mut VarDeclarator) {
        add_and_visit_if_in_span!(self, n);
    }
    // fn visit_mut_var_declarators(&mut self, n: &mut Vec<VarDeclarator>) { add_if_in_span!(self, n); }

    fn visit_mut_with_stmt(&mut self, n: &mut WithStmt) { add_and_visit_if_in_span!(self, n); }
    fn visit_mut_yield_expr(&mut self, n: &mut YieldExpr) { add_and_visit_if_in_span!(self, n); }

    /////////////////////////
    /// swc meta
    // fn visit_mut_atom(&mut self, n: &mut Atom) { add_if_in_span!(self, n); }
    // fn visit_mut_invalid(&mut self, n: &mut Invalid) { add_if_in_span!(self, n); }
    // fn visit_mut_js_word(&mut self, n: &mut JsWord) { add_if_in_span!(self, n); }
    // fn visit_mut_span(&mut self, n: &mut Span) { add_if_in_span!(self, n); }

    /// typescript
    // fn visit_mut_export_namespace_specifier(&mut self, n: &mut ExportNamespaceSpecifier) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_ts_entity_name(&mut self, n: &mut Option<TsEntityName>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_ts_namespace_body(&mut self, n: &mut Option<TsNamespaceBody>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_ts_type(&mut self, n: &mut Option<Box<TsType>>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_ts_type_ann(&mut self, n: &mut Option<Box<TsTypeAnn>>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_ts_type_param_decl(&mut self, n: &mut Option<Box<TsTypeParamDecl>>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_ts_type_param_instantiation(&mut self, n: &mut Option<Box<TsTypeParamInstantiation>>) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_array_type(&mut self, n: &mut TsArrayType) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_as_expr(&mut self, n: &mut TsAsExpr) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_call_signature_decl(&mut self, n: &mut TsCallSignatureDecl) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_conditional_type(&mut self, n: &mut TsConditionalType) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_const_assertion(&mut self, n: &mut TsConstAssertion) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_construct_signature_decl(&mut self, n: &mut TsConstructSignatureDecl) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_constructor_type(&mut self, n: &mut TsConstructorType) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_entity_name(&mut self, n: &mut TsEntityName) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_enum_decl(&mut self, n: &mut TsEnumDecl) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_enum_member(&mut self, n: &mut TsEnumMember) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_enum_member_id(&mut self, n: &mut TsEnumMemberId) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_enum_members(&mut self, n: &mut Vec<TsEnumMember>) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_export_assignment(&mut self, n: &mut TsExportAssignment) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_expr_with_type_args(&mut self, n: &mut TsExprWithTypeArgs) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_expr_with_type_args_vec(&mut self, n: &mut Vec<TsExprWithTypeArgs>) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_external_module_ref(&mut self, n: &mut TsExternalModuleRef) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_fn_or_constructor_type(&mut self, n: &mut TsFnOrConstructorType) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_fn_param(&mut self, n: &mut TsFnParam) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_fn_params(&mut self, n: &mut Vec<TsFnParam>) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_fn_type(&mut self, n: &mut TsFnType) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_getter_signature(&mut self, n: &mut TsGetterSignature) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_import_equals_decl(&mut self, n: &mut TsImportEqualsDecl) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_import_type(&mut self, n: &mut TsImportType) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_index_signature(&mut self, n: &mut TsIndexSignature) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_indexed_access_type(&mut self, n: &mut TsIndexedAccessType) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_infer_type(&mut self, n: &mut TsInferType) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_instantiation(&mut self, n: &mut TsInstantiation) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_interface_body(&mut self, n: &mut TsInterfaceBody) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_interface_decl(&mut self, n: &mut TsInterfaceDecl) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_intersection_type(&mut self, n: &mut TsIntersectionType) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_keyword_type(&mut self, n: &mut TsKeywordType) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_keyword_type_kind(&mut self, n: &mut TsKeywordTypeKind) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_lit(&mut self, n: &mut TsLit) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_lit_type(&mut self, n: &mut TsLitType) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_mapped_type(&mut self, n: &mut TsMappedType) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_method_signature(&mut self, n: &mut TsMethodSignature) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_module_block(&mut self, n: &mut TsModuleBlock) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_module_decl(&mut self, n: &mut TsModuleDecl) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_module_name(&mut self, n: &mut TsModuleName) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_module_ref(&mut self, n: &mut TsModuleRef) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_namespace_body(&mut self, n: &mut TsNamespaceBody) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_namespace_decl(&mut self, n: &mut TsNamespaceDecl) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_namespace_export_decl(&mut self, n: &mut TsNamespaceExportDecl) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_non_null_expr(&mut self, n: &mut TsNonNullExpr) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_optional_type(&mut self, n: &mut TsOptionalType) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_param_prop(&mut self, n: &mut TsParamProp) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_param_prop_param(&mut self, n: &mut TsParamPropParam) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_parenthesized_type(&mut self, n: &mut TsParenthesizedType) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_property_signature(&mut self, n: &mut TsPropertySignature) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_qualified_name(&mut self, n: &mut TsQualifiedName) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_rest_type(&mut self, n: &mut TsRestType) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_satisfies_expr(&mut self, n: &mut TsSatisfiesExpr) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_setter_signature(&mut self, n: &mut TsSetterSignature) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_this_type(&mut self, n: &mut TsThisType) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_this_type_or_ident(&mut self, n: &mut TsThisTypeOrIdent) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_tpl_lit_type(&mut self, n: &mut TsTplLitType) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_tuple_element(&mut self, n: &mut TsTupleElement) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_tuple_elements(&mut self, n: &mut Vec<TsTupleElement>) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_tuple_type(&mut self, n: &mut TsTupleType) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_type(&mut self, n: &mut TsType) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_type_alias_decl(&mut self, n: &mut TsTypeAliasDecl) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_type_ann(&mut self, n: &mut TsTypeAnn) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_type_assertion(&mut self, n: &mut TsTypeAssertion) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_type_element(&mut self, n: &mut TsTypeElement) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_type_elements(&mut self, n: &mut Vec<TsTypeElement>) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_type_lit(&mut self, n: &mut TsTypeLit) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_type_operator(&mut self, n: &mut TsTypeOperator) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_type_operator_op(&mut self, n: &mut TsTypeOperatorOp) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_type_param(&mut self, n: &mut TsTypeParam) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_type_param_decl(&mut self, n: &mut TsTypeParamDecl) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_type_param_instantiation(&mut self, n: &mut TsTypeParamInstantiation) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_type_params(&mut self, n: &mut Vec<TsTypeParam>) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_type_predicate(&mut self, n: &mut TsTypePredicate) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_type_query(&mut self, n: &mut TsTypeQuery) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_type_query_expr(&mut self, n: &mut TsTypeQueryExpr) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_type_ref(&mut self, n: &mut TsTypeRef) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_types(&mut self, n: &mut Vec<Box<TsType>>) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_union_or_intersection_type(&mut self, n: &mut TsUnionOrIntersectionType) { add_if_in_span!(self, n); }
    // fn visit_mut_ts_union_type(&mut self, n: &mut TsUnionType) { add_if_in_span!(self, n); }

    /// optional types
    // fn visit_mut_opt_accessibility(&mut self, n: &mut Option<Accessibility>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_atom(&mut self, n: &mut Option<Atom>) { add_if_in_span!(self, n); }
    fn visit_mut_opt_block_stmt(&mut self, n: &mut Option<BlockStmt>) {
        match n {
            None => {}
            Some(s) => {
                add_and_visit_if_in_span!(self, s);
            }
        }
    }
    fn visit_mut_opt_call(&mut self, n: &mut OptCall) { add_and_visit_if_in_span!(self, n); }
    // fn visit_mut_opt_catch_clause(&mut self, n: &mut Option<CatchClause>) { add_if_in_span!(self, n); }
    fn visit_mut_opt_chain_expr(&mut self, n: &mut OptChainExpr) { add_and_visit_if_in_span!(self, n); }
    // fn visit_mut_opt_chain_base(&mut self, n: &mut OptChainBase) { add_if_in_span!(self, n); }
    fn visit_mut_opt_expr(&mut self, n: &mut Option<Box<Expr>>) {
        match n {
            None => {}
            Some(x) => { x.visit_mut_with(self); }
        }
    }
    // fn visit_mut_opt_expr_or_spread(&mut self, n: &mut Option<ExprOrSpread>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_expr_or_spreads(&mut self, n: &mut Option<Vec<ExprOrSpread>>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_ident(&mut self, n: &mut Option<Ident>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_jsx_attr_value(&mut self, n: &mut Option<JSXAttrValue>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_jsx_closing_element(&mut self, n: &mut Option<JSXClosingElement>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_module_export_name(&mut self, n: &mut Option<ModuleExportName>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_module_items(&mut self, n: &mut Option<Vec<ModuleItem>>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_object_lit(&mut self, n: &mut Option<Box<ObjectLit>>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_pat(&mut self, n: &mut Option<Pat>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_span(&mut self, n: &mut Option<Span>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_stmt(&mut self, n: &mut Option<Box<Stmt>>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_str(&mut self, n: &mut Option<Box<Str>>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_true_plus_minus(&mut self, n: &mut Option<TruePlusMinus>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_var_decl_or_expr(&mut self, n: &mut Option<VarDeclOrExpr>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_vec_expr_or_spreads(&mut self, n: &mut Vec<Option<ExprOrSpread>>) { add_if_in_span!(self, n); }
    // fn visit_mut_opt_vec_pats(&mut self, n: &mut Vec<Option<Pat>>) { add_if_in_span!(self, n); }

    // jsx
    // fn visit_mut_jsx_attr(&mut self, n: &mut JSXAttr) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_attr_name(&mut self, n: &mut JSXAttrName) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_attr_or_spread(&mut self, n: &mut JSXAttrOrSpread) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_attr_or_spreads(&mut self, n: &mut Vec<JSXAttrOrSpread>) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_attr_value(&mut self, n: &mut JSXAttrValue) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_closing_element(&mut self, n: &mut JSXClosingElement) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_closing_fragment(&mut self, n: &mut JSXClosingFragment) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_element(&mut self, n: &mut JSXElement) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_element_child(&mut self, n: &mut JSXElementChild) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_element_children(&mut self, n: &mut Vec<JSXElementChild>) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_element_name(&mut self, n: &mut JSXElementName) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_empty_expr(&mut self, n: &mut JSXEmptyExpr) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_expr(&mut self, n: &mut JSXExpr) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_expr_container(&mut self, n: &mut JSXExprContainer) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_fragment(&mut self, n: &mut JSXFragment) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_member_expr(&mut self, n: &mut JSXMemberExpr) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_namespaced_name(&mut self, n: &mut JSXNamespacedName) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_object(&mut self, n: &mut JSXObject) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_opening_element(&mut self, n: &mut JSXOpeningElement) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_opening_fragment(&mut self, n: &mut JSXOpeningFragment) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_spread_child(&mut self, n: &mut JSXSpreadChild) { add_if_in_span!(self, n); }
    // fn visit_mut_jsx_text(&mut self, n: &mut JSXText) { add_if_in_span!(self, n); }
}

impl FindNeighbors {
    fn is_in(&mut self, span: Span) -> bool {
        // debug!("CHECK: is {} in {}-{}", self.target_loc, span.lo.0, span.hi.0);
        // println!("CHECK: is {} in {}-{}", self.target_loc, span.lo.0, span.hi.0);
        self.target_loc >= span.lo.0 && self.target_loc < span.hi.0
    }
}
