use swc_core::{
    common::{util::take::Take, DUMMY_SP},
    ecma::ast::{
        ArrayLit, ArrowExpr, BlockStmt, BlockStmtOrExpr, Expr, Ident,
        JSXAttrName, JSXNamespacedName, Stmt,
    },
    quote,
};

use crate::shared::take_array_item;

pub enum Binding {
    Eval {
        name: Ident,
        value: Expr,
    },
    Signal {
        name: Ident,
        signal: Expr,
    },
    Event {
        name: Ident,
        listener: Expr,
        opts: Option<Expr>,
    },
}

impl Binding {
    pub fn new(name: &mut JSXAttrName, expr: Expr) -> Self {
        match name {
            JSXAttrName::Ident(id) => {
                Binding::Eval {
                    name: id.take(),
                    value: expr,
                }
            },
            JSXAttrName::JSXNamespacedName(JSXNamespacedName {
                ns,
                name,
            }) => {
                match &*ns.sym {
                    "s" => {
                        Binding::Signal {
                            name: name.take(),
                            signal: expr,
                        }
                    },
                    "on" => {
                        match expr {
                            Expr::Array(ArrayLit {
                                mut elems, ..
                            }) => {
                                let listener = take_array_item(&mut elems[0]);
                                let opts = take_array_item(&mut elems[1]);

                                Binding::Event {
                                    name: name.take(),
                                    listener,
                                    opts: Some(opts),
                                }
                            },
                            _ => {
                                Binding::Event {
                                    name: name.take(),
                                    listener: expr,
                                    opts: None,
                                }
                            },
                        }
                    },
                    _ => {
                        todo!("JSXAttrName::JSXNamespacedName")
                    },
                }
            },
        }
    }

    fn eval_stmt(el: Ident, name: Ident, value: Expr) -> Stmt {
        match &*name.sym {
            "ref" => {
                quote!(
                    "$ref_var.value = $el" as Stmt,
                    ref_var: Expr = value,
                    el = el
                )
            },
            "text" => {
                quote!(
                    "$el.textContent = $value" as Stmt,
                    el = el,
                    value: Expr = value
                )
            },
            "html" => {
                quote!(
                    "$el.innerHTML = $value" as Stmt,
                    el = el,
                    value: Expr = value
                )
            },
            "class" => {
                quote!(
                    "$el.className = $value" as Stmt,
                    el = el,
                    value: Expr = value
                )
            },
            "style" => {
                quote!(
                    "$el.style.cssText = $value" as Stmt,
                    el = el,
                    value: Expr = value
                )
            },
            _ => {
                quote!(
                    "$el.setAttribute($name, $value)" as Stmt,
                    el = el,
                    name: Expr = name.sym.into(),
                    value: Expr = value
                )
            },
        }
    }

    pub fn stmt(self, el: Ident) -> Stmt {
        match self {
            Self::Eval {
                name,
                value,
            } => Self::eval_stmt(el, name, value),
            Self::Signal {
                name,
                signal,
            } => {
                let value = quote!(
                    "$signal.value" as Expr,
                    signal: Expr = signal.clone()
                );

                let callback = ArrowExpr {
                    body: BlockStmtOrExpr::BlockStmt(BlockStmt {
                        span: DUMMY_SP,
                        stmts: vec![Self::eval_stmt(el, name, value)],
                    }),
                    ..Take::dummy()
                };

                quote!(
                    "$signal.subscribe($callback)" as Stmt,
                    signal: Expr = signal,
                    callback: Expr = callback.into(),
                )
            },
            Self::Event {
                name,
                listener,
                opts: None,
            } => {
                quote!(
                    "$el.addEventListener($name,$listener)" as Stmt,
                    el = el,
                    name: Expr = name.sym.into(),
                    listener: Expr = listener
                )
            },
            Self::Event {
                name,
                listener,
                opts: Some(opts),
            } => {
                quote!(
                    "$el.addEventListener($name,$listener,$opts)" as Stmt,
                    el = el,
                    name: Expr = name.sym.into(),
                    listener: Expr = listener,
                    opts: Expr = opts
                )
            },
        }
    }
}
