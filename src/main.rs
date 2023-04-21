use darling::{export::syn, util::PathList, FromMeta};
use quote;
#[derive(Debug, FromMeta)]
struct EncapsulationArgs {
    #[darling(multiple, rename = "encapsulated_type")]
    types: PathList,
    #[darling(rename = "encapsulated_struct_name")]
    name: Option<String>,
}

#[derive(Debug, FromMeta)]
struct ExportMethodsArgs {
    #[darling(multiple, rename = "exported_method")]
    methods: Vec<String>,
}

#[derive(Debug, FromMeta)]
#[darling(rename_all = "snake_case")]
enum MacroArgs {
    Encapsulation(EncapsulationArgs),
    ExportMethods(ExportMethodsArgs),
}

fn main() {
    let input = quote::quote! {
        #[encapsulation(Knobs, Database, knobs)]
        pub(crate) struct KnobsStruct {
            #[export_methods(
                fn signal(&self, stage: usize),
                fn wait_for(&self, stage: usize)
            )]
            pub signal: Arc<Signal>,
            pub signal_on_will_block: Cell<usize>,
        }
    };
    let ast = syn::parse2(input).unwrap();
    let attrs = ast
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("encapsulation"));
    for attr in attrs {
        let args = MacroArgs::from_list(&attr.tokens).unwrap();
        println!("{:?}", args);
    }
}
