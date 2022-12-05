use swc_core::{
    common::{sync::Lrc, util::take::Take, SourceMap, DUMMY_SP},
    ecma::{
        ast::{Expr, JSXElement, JSXElementChild, JSXText, Tpl, TplElement},
        codegen::{text_writer::JsWriter, Config, Emitter, Node},
        visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
    },
};

#[derive(Default)]
pub struct JSX2Html {
    code_buffer: Vec<u8>,
    cm: Lrc<SourceMap>,
}

impl JSX2Html {
    fn emit_code<N: Node>(&mut self, node: &N) -> String {
        let mut emitter = Emitter {
            cfg: Config::default(),
            cm: self.cm.clone(),
            comments: None,
            wr: JsWriter::new(
                self.cm.clone(),
                "\n",
                &mut self.code_buffer,
                None,
            ),
        };

        node.emit_with(&mut emitter).unwrap();

        String::from_utf8(self.code_buffer.take()).unwrap()
    }

    fn html_tpl(&mut self, elm: &JSXElement) -> Tpl {
        let html = self.emit_code(elm);

        Tpl {
            quasis: vec![TplElement {
                raw: html.into(),
                ..Take::dummy()
            }],
            ..Take::dummy()
        }
    }
}

impl VisitMut for JSX2Html {
    noop_visit_mut_type!();

    fn visit_mut_jsx_element_child(&mut self, node: &mut JSXElementChild) {
        if let JSXElementChild::JSXExprContainer(container) = node {
            let code = self.emit_code(container);

            let interpolation = format!("${}", code);

            *node = JSXElementChild::JSXText(JSXText {
                span: DUMMY_SP,
                value: interpolation.into(),
                raw: Default::default(),
            })
        }

        node.visit_mut_children_with(self);
    }

    fn visit_mut_expr(&mut self, node: &mut Expr) {
        node.visit_mut_children_with(self);

        match node {
            Expr::JSXElement(box elm) => *node = self.html_tpl(elm).into(),
            Expr::JSXFragment(_) => {
                todo!("Expr::JSXFragment")
            },
            _ => {},
        }
    }
}
