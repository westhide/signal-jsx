use swc_core::{
    common::{util::take::Take, DUMMY_SP},
    ecma::{
        ast::{
            CallExpr, Callee, Expr, Ident, JSXAttr, JSXAttrName,
            JSXAttrOrSpread, JSXAttrValue, JSXExpr, JSXExprContainer,
            KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread,
        },
        utils::private_ident,
    },
};

use crate::regex;

pub struct Component {
    pub interval: usize,
    pub id: Ident,
    pub tag: Ident,
    pub props: Vec<PropOrSpread>,
}

impl Component {
    pub fn new(
        interval: usize,
        tag: Ident,
        attrs: Vec<JSXAttrOrSpread>,
    ) -> Self {
        Self {
            interval,
            id: private_ident!("$cmpt"),
            tag,
            props: Self::take_props(attrs),
        }
    }

    pub fn is(tag: &Ident) -> bool {
        regex!("^[A-Z]").is_match(&tag.sym)
    }

    pub fn call_expr(tag: Ident, props: Vec<PropOrSpread>) -> CallExpr {
        let props_obj = ObjectLit {
            span: DUMMY_SP,
            props,
        };

        CallExpr {
            callee: Callee::Expr(box tag.into()),
            args: vec![Expr::Object(props_obj).into()],
            ..Take::dummy()
        }
    }

    fn key_value_prop(key: Ident, value: Box<Expr>) -> PropOrSpread {
        Prop::KeyValue(KeyValueProp {
            key: PropName::Ident(key),
            value,
        })
        .into()
    }

    fn take_props(attrs: Vec<JSXAttrOrSpread>) -> Vec<PropOrSpread> {
        let mut props = vec![];

        for attr_or_spread in attrs {
            match attr_or_spread {
                JSXAttrOrSpread::JSXAttr(JSXAttr {
                    name: JSXAttrName::Ident(key),
                    value: Some(value),
                    ..
                }) => {
                    match value {
                        JSXAttrValue::Lit(lit) => {
                            props.push(Self::key_value_prop(key, lit.into()))
                        },

                        JSXAttrValue::JSXExprContainer(JSXExprContainer {
                            expr,
                            ..
                        }) => {
                            match expr {
                                JSXExpr::Expr(value) => {
                                    props.push(Self::key_value_prop(key, value))
                                },
                                JSXExpr::JSXEmptyExpr(_) => {},
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
                JSXAttrOrSpread::JSXAttr(_) => {},
                JSXAttrOrSpread::SpreadElement(_) => {
                    todo!("JSXAttr::SpreadElement")
                },
            };
        }

        props
    }
}
