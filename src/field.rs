use darling::{
    export::syn::{
        self, parse_macro_input, punctuated::Pair, Attribute, Field, Path, PathArguments,
        PathSegment, ReturnType, Token,
    },
    FromMeta, ToTokens,
};

use crate::*;

#[derive(Debug, FromField)]
#[darling(attributes(export_methods))]
pub(crate) struct EncapsulationField {
    ident: Option<Ident>,
    // attrs: Vec<Attribute>,
}

// #[derive(Debug, FromMeta)]
// // #[darling(attributes(export_methods))]
// pub(crate) struct EncapsulationFieldMetaItem {
//     func: TraitItemMethod,
// }

// export_methods: Vec<TraitItemMethod>, // vis: Visibility,
// ty: Type,
// attrs: Vec<Attribute>,
mod input_bad {
    use super::*;
    #[test]
    fn test_parse_field_with_empty_attrs() {
        #[allow(dead_code)]
        #[derive(Debug, FromField)]
        #[darling(attributes(encapsulation))]
        pub(crate) struct EncapsulationField {
            ident: Option<Ident>,
            attrs: Vec<Attribute>,
        }
        let field = handle_field(handle_fields(get_input_bad()));
        assert_debug_snapshot!(field.pretty());
        let field = EncapsulationField::from_field(&field).unwrap();
        assert_debug_snapshot!(field);
    }

    #[test]
    fn test_parse_attr_bad() {
        let attr = get_attr_bad();
        // assert_debug_snapshot!(token_to_string(attr.to_token_stream()));
        assert_debug_snapshot!(attr.pretty());
        assert_debug_snapshot!(attr);
        let meta = attr.parse_meta();
        assert_debug_snapshot!(meta); // "expected `,`",

        // try a debug
        debug::get_path();
        // debug::tokens_to_parse_buffer(); // state=tokens_to_parse_buffer(&buf);
        // debug::parse_meta_after_path();// self(&state)?
        pub mod debug {
            use std::{cell::Cell, rc::Rc};

            use darling::export::syn::{buffer::TokenBuffer, parse::ParseBuffer};

            use super::*;
            pub fn parse2() {
                fn tokens_to_parse_buffer(tokens: &TokenBuffer) -> ParseBuffer {
                    // pub(crate) fn new_parse_buffer(
                    //     scope: Span,
                    //     cursor: Cursor,
                    //     unexpected: Rc<Cell<Unexpected>>,
                    // ) -> ParseBuffer {
                    //     ParseBuffer {
                    //         scope,
                    //         // See comment on `cell` in the struct definition.
                    //         cell: Cell::new(unsafe {
                    //             mem::transmute::<Cursor, Cursor<'static>>(cursor)
                    //         }),
                    //         marker: PhantomData,
                    //         unexpected: Cell::new(Some(unexpected)),
                    //     }
                    // }
                    let scope = Span::call_site();
                    let cursor = tokens.begin();
                    let unexpected = Rc::new(Cell::new(Unexpected::None));
                    new_parse_buffer(scope, cursor, unexpected)
                }
                let path = get_path();
                let tokens = get_attr_bad().tokens;
                let buf = syn::buffer::TokenBuffer::new2(tokens);
                let state = tokens_to_parse_buffer(&buf);
            }
            pub fn get_path() -> Path {
                pub fn parse_meta(attr: Attribute) -> Path {
                    fn clone_ident_segment(segment: &PathSegment) -> PathSegment {
                        PathSegment {
                            ident: segment.ident.clone(),
                            arguments: PathArguments::None,
                        }
                    }

                    let path = Path {
                        leading_colon: attr
                            .path
                            .leading_colon
                            .as_ref()
                            .map(|colon| Token![::](colon.spans)),
                        segments: attr
                            .path
                            .segments
                            .pairs()
                            .map(|pair| match pair {
                                Pair::Punctuated(seg, punct) => Pair::Punctuated(
                                    clone_ident_segment(seg),
                                    Token![::](punct.spans),
                                ),
                                Pair::End(seg) => Pair::End(clone_ident_segment(seg)),
                            })
                            .collect(),
                    };
                    assert_debug_snapshot!(attr.tokens.pretty(),@r###""(fn signal (& self , stage : usize) , fn wait_for (& self , stage : usize) ,)""###);

                    path
                }
                let parse_meta_path = parse_meta(get_attr_bad());
                assert_debug_snapshot!(parse_meta_path.pretty(),@r###""encapsulation""###);
                parse_meta_path
            }
        }
    }
}
#[test]
fn test_parse_field_with_string() {
    #[allow(dead_code)]
    #[derive(Debug, FromField)]
    #[darling(attributes(encapsulation))]
    struct EncapsulationField {
        ident: Option<Ident>,
        #[darling(multiple, rename = "export_methods")]
        export_methods_str: Vec<String>,
        #[darling(skip)]
        export_methods: Vec<TraitItemMethod>,
    }
    assert_debug_snapshot!(get_field().pretty());
    let encapsulation_field = EncapsulationField::from_field(&get_field()).unwrap();
    assert_debug_snapshot!(encapsulation_field);
}

#[test]
fn test_parse_field() {
    //     ---- field::test_parse_field_with_empty_attrs stdout ----
    // thread 'field::test_parse_field_with_empty_attrs' panicked at 'called `Result::unwrap()` on an `Err` value: Error { kind: Custom("Unable to parse attribute: expected `,`"), locations: [], span: Some(Span) }', src/field.rs:23:62
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    let field = EncapsulationField::from_field(&get_field()).unwrap();
    dbg!("{:#?}", field);
}
