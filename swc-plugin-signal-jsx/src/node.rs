use swc_core::{
    common::util::take::Take,
    ecma::ast::{
        Expr, JSXAttr, JSXAttrName, JSXAttrOrSpread, JSXAttrValue, JSXExpr,
        JSXExprContainer,
    },
};

use crate::{binding::Binding, hydration::Hydration};

#[derive(Default)]
pub struct Node {
    pub interval: usize,
    pub bindings: Vec<Binding>,
    pub children: Vec<Hydration>,
}

impl Node {
    pub fn is_empty(&self) -> bool {
        let Self {
            bindings,
            children,
            ..
        } = self;

        bindings.is_empty() && children.is_empty()
    }

    fn bind(&mut self, name: &mut JSXAttrName, expr: Expr) {
        self.bindings.push(Binding::new(name, expr))
    }

    pub fn take_bindings(&mut self, attrs: &mut Vec<JSXAttrOrSpread>) {
        attrs.retain_mut(|attr: &mut JSXAttrOrSpread| {
            match attr {
                JSXAttrOrSpread::JSXAttr(JSXAttr {
                    name,
                    value: Some(value),
                    ..
                }) => {
                    match value {
                        JSXAttrValue::Lit(_) => true,

                        JSXAttrValue::JSXExprContainer(JSXExprContainer {
                            expr: jsx_expr,
                            ..
                        }) => {
                            match jsx_expr {
                                JSXExpr::JSXEmptyExpr(_) => false,
                                JSXExpr::Expr(box Expr::Lit(lit)) => {
                                    *value = lit.clone().into();

                                    true
                                },
                                JSXExpr::Expr(box expr) => {
                                    self.bind(name, expr.take());

                                    false
                                },
                            }
                        },

                        JSXAttrValue::JSXElement(_) => {
                            todo!("JSXAttr::JSXElement")
                        },

                        JSXAttrValue::JSXFragment(_) => {
                            todo!("JSXAttr::JSXElement")
                        },
                    }
                },
                JSXAttrOrSpread::JSXAttr(JSXAttr {
                    value: None, ..
                }) => true,
                JSXAttrOrSpread::SpreadElement(_) => {
                    todo!("JSXAttr::SpreadElement")
                },
            }
        })
    }
}
