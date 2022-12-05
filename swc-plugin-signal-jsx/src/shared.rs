use swc_core::{
    common::util::take::Take,
    ecma::ast::{Expr, ExprOrSpread},
};

pub fn take_array_item(item: &mut Option<ExprOrSpread>) -> Expr {
    let Some(item) = item else { panic!("Error: Expr::Array can not be None") };

    if item.spread.is_some() {
        panic!("Expr::Array spread nonsupport")
    };

    *item.expr.take()
}
