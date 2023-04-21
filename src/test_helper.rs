#![cfg(test)]
use darling::{
    export::syn::{Attribute, Data, Field, Fields},
    ToTokens,
};
use proc_macro2::TokenStream;

use crate::*;
pub(crate) fn get_input_bad() -> DeriveInput {
    parse_quote! {
        #[derive(Encapsulation)]
        #[encapsulation(encapsulant_name = "Database")]
        pub(crate) struct KnobsStruct {
            #[export_methods(
                fn signal(&self, stage: usize),
                fn wait_for(&self, stage: usize),
            )]
            pub signal: Arc<Signal>,
            pub signal_on_will_block: Cell<usize>,
        }
    }
}
pub(crate) fn get_attr_bad() -> Attribute {
    let attr = handle_attr(handle_field(handle_fields(get_input_bad())));
    attr
}

pub(crate) fn get_input() -> DeriveInput {
    parse_quote! {
        #[derive(Encapsulation)]
        #[encapsulation(encapsulant_name = "Database")]
        pub(crate) struct KnobsStruct {
            #[encapsulation(export_methods="fn signal(&self, stage: usize)")]
            pub signal: Arc<Signal>,
            pub signal_on_will_block: Cell<usize>,
        }
    }
}
pub(crate) fn handle_fields(input: DeriveInput) -> Fields {
    match input.data {
        Data::Struct(obj) => obj.fields,
        Data::Enum(_) => todo!(),
        Data::Union(_) => todo!(),
    }
}
pub(crate) fn get_fields() -> Fields {
    handle_fields(get_input())
}

pub(crate) fn handle_field(fields: Fields) -> Field {
    fields.iter().next().unwrap().clone()
}
pub(crate) fn get_field() -> Field {
    handle_field(get_fields())
}
pub(crate) fn handle_attr(field: Field) -> Attribute {
    field.attrs.iter().next().unwrap().clone()
}
pub(crate) fn get_attr() -> Attribute {
    handle_attr(get_field())
}

pub(crate) fn token_to_string(tokens: TokenStream) -> String {
    tokens
        .into_iter()
        .map(|token| token.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

pub(crate) trait PrettyDebug: ToTokens {
    fn pretty(&self) -> String {
        token_to_string(self.to_token_stream())
    }
}

impl<T> PrettyDebug for T where T: ToTokens {}
