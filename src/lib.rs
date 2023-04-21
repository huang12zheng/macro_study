mod field;
#[cfg(test)]
pub(crate) use darling::export::syn::parse_quote;
pub(crate) use darling::{
    export::syn::{DeriveInput, TraitItemMethod, Type},
    FromDeriveInput, FromField,
};
#[cfg(test)]
pub(crate) use insta::assert_debug_snapshot;
// use proc_macro2::TokenStream;
pub(crate) use convert_case::{Case, Casing};
pub(crate) use darling::export::syn::{Ident, Visibility};
pub(crate) use proc_macro2::Span;

// #[allow(dead_code)]
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(encapsulation), forward_attrs(allow, doc, cfg))]
pub struct Encapsulation {
    ident: Ident,
    vis: Visibility,
    trait_name: Option<Ident>,
    encapsulant_name: Ident,
    property_name: Option<Ident>,
}

impl Encapsulation {
    pub fn from_input(input: &DeriveInput) -> Result<Self, darling::Error> {
        Encapsulation::from_derive_input(input).and_then(|mut encapsulation| {
            if encapsulation.trait_name.is_none() {
                let encapsulation_str = encapsulation.ident.to_string();
                let trait_name = if encapsulation_str.ends_with("Struct") {
                    encapsulation_str.trim_end_matches("Struct").to_owned()
                } else {
                    format!("I{}", encapsulation_str.to_owned())
                };
                encapsulation.trait_name = Some(Ident::new(&trait_name, Span::call_site()));
            }
            if encapsulation.property_name.is_none() {
                let encapsulation_str = encapsulation.ident.to_string();
                let property_name = if encapsulation_str.ends_with("Struct") {
                    encapsulation_str.trim_end_matches("Struct").to_owned()
                } else {
                    format!("{}", encapsulation_str.to_owned())
                }
                .to_case(Case::Snake);
                encapsulation.property_name = Some(Ident::new(&property_name, Span::call_site()));
            }
            Ok(encapsulation)
        })
    }
}

#[test]
fn test_parse_fail() {
    // 输入包含 `#[encapsulation(...)]` 和 `#[export_methods(...)]` 注解
    let input = parse_quote! {
        #[derive(Encapsulation)]
        #[encapsulation(encapsulant_name = Database)]
        pub(crate) struct KnobsStruct {
        }
    };

    let actual = Encapsulation::from_derive_input(&input);
    assert_debug_snapshot!(actual.is_err());
    assert_debug_snapshot!(actual);
}
#[test]
fn test_parse_input() {
    // 输入包含 `#[encapsulation(...)]` 和 `#[export_methods(...)]` 注解
    let input = parse_quote! {
        #[derive(Encapsulation)]
        #[encapsulation(encapsulant_name = "Database")]
        pub(crate) struct KnobsStruct {
            #[export_methods(
                fn signal(&self, stage: usize),
                fn wait_for(&self, stage: usize)
            )]
            pub signal: Arc<Signal>,
            pub signal_on_will_block: Cell<usize>,
        }
    };

    let actual = Encapsulation::from_derive_input(&input).unwrap();
    assert_debug_snapshot!(actual);
    let actual = Encapsulation::from_input(&input).unwrap();
    assert_debug_snapshot!(actual);
}
// struct MethodList(Vec<syn::TraitItemMethod>);

mod test_helper;
#[cfg(test)]
pub use test_helper::*;
