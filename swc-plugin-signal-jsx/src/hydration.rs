use swc_core::{
    common::{util::take::Take, DUMMY_SP},
    ecma::{
        ast::{
            ArrowExpr, BlockStmt, BlockStmtOrExpr, Ident, JSXElement,
            JSXElementChild, JSXElementName, JSXExpr, JSXExprContainer,
            JSXText, Stmt,
        },
        utils::{private_ident, quote_ident},
    },
    quote,
};

use crate::{
    binding::Binding, component::Component, import::ImportMap, node::Node,
};

pub enum Hydration {
    Node(Node),
    Component(Component),
}

impl Hydration {
    pub fn parse_jsx(
        mut interval: usize,
        elm: &mut JSXElement,
    ) -> Option<Self> {
        let mut node = Node {
            interval,
            ..Default::default()
        };

        let JSXElement {
            opening,
            children,
            ..
        } = elm;

        node.take_bindings(&mut opening.attrs);

        interval = 0;

        for child in children {
            match child {
                // trim JSXText
                JSXElementChild::JSXText(JSXText {
                    value, ..
                }) => {
                    *value = value.trim().into();

                    if !value.is_empty() {
                        interval += 1
                    };
                },

                JSXElementChild::JSXExprContainer(container) => {
                    match &mut container.expr {
                        JSXExpr::JSXEmptyExpr(_) => {},
                        JSXExpr::Expr(box signal) => {
                            let hydration = Hydration::Node(Node {
                                interval,
                                bindings: vec![Binding::Signal {
                                    name: quote_ident!("text"),
                                    signal: signal.take(),
                                }],
                                children: vec![],
                            });

                            node.children.push(hydration)
                        },
                    }

                    *child = JSXElementChild::JSXText(JSXText {
                        span: DUMMY_SP,
                        value: " ".into(),
                        raw: Default::default(),
                    });

                    interval = 1
                },

                JSXElementChild::JSXSpreadChild(_) => {
                    todo!("JSXElementChild::JSXSpreadChild")
                },

                JSXElementChild::JSXElement(box elm) => {
                    match &mut elm.opening.name {
                        JSXElementName::Ident(tag) if Component::is(tag) => {
                            let cmpt = Component::new(
                                interval,
                                tag.take(),
                                elm.opening.attrs.take(),
                            );

                            let html = quote!(
                                "$id.html" as Expr,
                                id = cmpt.id.clone()
                            );

                            *child = JSXElementChild::JSXExprContainer(
                                JSXExprContainer {
                                    span: DUMMY_SP,
                                    expr: JSXExpr::Expr(box html),
                                },
                            );

                            node.children.push(Hydration::Component(cmpt));
                            interval = 1
                        },
                        JSXElementName::Ident(_) => {
                            match Hydration::parse_jsx(interval, elm) {
                                Some(hydration) => {
                                    node.children.push(hydration);
                                    interval = 1
                                },
                                None => interval += 1,
                            }
                        },
                        JSXElementName::JSXNamespacedName(_) => {
                            todo!("JSXElementName::JSXNamespacedName")
                        },
                        JSXElementName::JSXMemberExpr(_) => {
                            todo!("JSXElementName::JSXMemberExpr")
                        },
                    };
                },

                JSXElementChild::JSXFragment(_) => {
                    todo!("JSXElementChild::JSXFragment")
                },
            }
        }

        if node.is_empty() {
            None
        } else {
            Some(Hydration::Node(node))
        }
    }
}

pub trait Patch {
    fn ident() -> Ident {
        private_ident!("$el")
    }

    fn el_ident(
        im: &mut ImportMap,
        interval: usize,
        cursor: Ident,
        stmts: &mut Vec<Stmt>,
    ) -> Ident {
        match interval {
            0 => cursor,
            _ => {
                let el = Self::ident();

                let stmt = if interval == 1 {
                    quote!(
                        "const $el = $cursor.nextSibling" as Stmt,
                        el = el.clone(),
                        cursor = cursor
                    )
                } else {
                    let to_sibling = im.get("toSibling");

                    quote!(
                        "const $el = $to_sibling($cursor,$interval)" as Stmt,
                        el = el.clone(),
                        to_sibling = to_sibling,
                        cursor = cursor,
                        interval: Expr = interval.into()
                    )
                };

                stmts.push(stmt);
                el
            },
        }
    }

    fn patch_stmts(
        self,
        im: &mut ImportMap,
        cursor: Ident,
        stmts: &mut Vec<Stmt>,
        decls: &mut Vec<Stmt>,
    ) -> Ident;

    fn patch(self, im: &mut ImportMap) -> (Vec<Stmt>, ArrowExpr);
}

impl Patch for Hydration {
    fn patch_stmts(
        self,
        im: &mut ImportMap,
        cursor: Ident,
        stmts: &mut Vec<Stmt>,
        decls: &mut Vec<Stmt>,
    ) -> Ident {
        match self {
            Hydration::Node(node) => {
                let Node {
                    interval,
                    bindings,
                    children,
                } = node;

                let el = Hydration::el_ident(im, interval, cursor, stmts);

                for binding in bindings {
                    stmts.push(binding.stmt(el.clone()))
                }

                if !children.is_empty() {
                    let mut cursor = Hydration::ident();

                    let stmt = quote!(
                        "const $cursor = $el.firstChild" as Stmt,
                        cursor = cursor.clone(),
                        el = el.clone(),
                    );

                    stmts.push(stmt);

                    for child in children {
                        cursor = child.patch_stmts(im, cursor, stmts, decls)
                    }
                }
                el
            },

            Hydration::Component(cmpt) => {
                let Component {
                    interval,
                    id,
                    tag,
                    props,
                } = cmpt;

                let el = Hydration::el_ident(im, interval, cursor, stmts);

                let call_expr = Component::call_expr(tag, props);

                let decl = quote!(
                    "const $cmpt = $call_expr" as Stmt,
                    cmpt = id.clone(),
                    call_expr: Expr = call_expr.into()
                );

                decls.push(decl);

                let stmt = quote!(
                    "$cmpt.patch($el)" as Stmt,
                    cmpt = id,
                    el = el.clone(),
                );

                stmts.push(stmt);

                el
            },
        }
    }

    fn patch(self, im: &mut ImportMap) -> (Vec<Stmt>, ArrowExpr) {
        let el = Hydration::ident();

        let mut stmts = vec![];
        let mut decls = vec![];

        self.patch_stmts(im, el.clone(), &mut stmts, &mut decls);

        let func = ArrowExpr {
            params: vec![el.into()],
            body: BlockStmtOrExpr::BlockStmt(BlockStmt {
                span: DUMMY_SP,
                stmts,
            }),
            ..Take::dummy()
        };

        (decls, func)
    }
}
