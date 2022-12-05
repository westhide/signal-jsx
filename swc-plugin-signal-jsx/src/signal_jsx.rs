use swc_core::{
    common::util::take::Take,
    ecma::{
        ast::{
            ArrowExpr, BlockStmt, BlockStmtOrExpr, Expr, Ident, JSXElementName,
            Module,
        },
        visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
    },
    quote,
};

use crate::{
    component::Component,
    hydration::{Hydration, Patch},
    import::ImportMap,
};

#[derive(Default)]
pub struct SignalJSX<'a> {
    pub import_map: ImportMap<'a>,
}

impl<'a> SignalJSX<'a> {
    fn cmpt_ident(&mut self) -> Ident {
        self.import_map.get("component$")
    }
}

impl<'a> VisitMut for SignalJSX<'a> {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, module: &mut Module) {
        module.visit_mut_children_with(self);

        self.import_map.gen_import_decl(module)
    }

    /// JSX entry
    fn visit_mut_expr(&mut self, node: &mut Expr) {
        node.visit_mut_children_with(self);

        match node {
            Expr::JSXElement(box elm) => {
                match &elm.opening.name {
                    JSXElementName::Ident(tag) if !Component::is(tag) => {
                        let Some(hydration) = Hydration::parse_jsx(0, elm) else {
                            todo!("static JSX Template")
                        };

                        let (mut decls, patch_fn) = hydration.patch_info();

                        let return_stmt = quote!(
                            "return $cmpt($html,$patch_fn)" as Stmt,
                            cmpt = self.cmpt_ident(),
                            html: Expr = Expr::JSXElement(box elm.take()),
                            patch_fn: Expr = patch_fn.into(),
                        );

                        let mut block = BlockStmt::dummy();

                        block.stmts.append(&mut decls);
                        block.stmts.push(return_stmt);

                        let func = ArrowExpr {
                            body: BlockStmtOrExpr::BlockStmt(block),
                            ..Take::dummy()
                        };

                        *node = quote!(
                            "($func)()" as Expr,
                            func: Expr = func.into()
                        );
                    },
                    _ => {
                        todo!("JSX entry")
                    },
                }
            },

            Expr::JSXFragment(_) => {
                todo!("Expr::JSXFragment")
            },
            _ => {},
        }
    }
}
