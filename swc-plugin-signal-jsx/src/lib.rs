#![feature(box_syntax)]
#![feature(box_patterns)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

pub use jsx2html::JSX2Html;
pub use signal_jsx::SignalJSX;
use swc_core::{
    common::chain,
    ecma::{
        ast::Program,
        transforms::base::hygiene::hygiene,
        visit::{as_folder, FoldWith},
    },
    plugin::{
        metadata::TransformPluginProgramMetadata as Metadata, plugin_transform,
    },
};

mod binding;
mod component;
mod hydration;
mod import;
mod jsx2html;
mod node;
mod regex_macro;
mod shared;
mod signal_jsx;

#[plugin_transform]
pub fn process_transform(program: Program, _: Metadata) -> Program {
    program.fold_with(&mut chain!(
        as_folder(SignalJSX::default()),
        hygiene(),
        as_folder(JSX2Html::default()),
    ))
}
