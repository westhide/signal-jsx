use std::collections::HashMap;

use swc_core::{
    common::{util::take::Take, DUMMY_SP},
    ecma::{
        ast::{
            Ident, ImportDecl, ImportNamedSpecifier, ImportSpecifier, Module,
            ModuleDecl, ModuleItem,
        },
        utils::private_ident,
    },
};

#[derive(Debug, Default)]
pub struct ImportMap<'a> {
    store: HashMap<&'a str, Ident>,
}

impl<'a> ImportMap<'a> {
    pub fn get(&mut self, name: &'a str) -> Ident {
        match self.store.get(name) {
            Some(id) => id.clone(),
            None => {
                let id = private_ident!(name);
                self.store.insert(name, id.clone());
                id
            },
        }
    }

    pub fn gen_import_decl(&mut self, module: &mut Module) {
        if self.store.is_empty() {
            return;
        }

        let import_decl = ImportDecl {
            specifiers: self
                .store
                .values()
                .map(|id| {
                    ImportSpecifier::Named(ImportNamedSpecifier {
                        span: DUMMY_SP,
                        local: id.clone(),
                        imported: None,
                        is_type_only: Default::default(),
                    })
                })
                .collect(),
            src: box "@westhide/tai".into(),
            ..Take::dummy()
        };

        let item = ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl));

        module.body.insert(0, item)
    }
}
