#![recursion_limit="128"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;

mod field;
use field::{GraphElemField, BuildSetters, BuildExistingFieldAdder, BuildSerializer};

#[proc_macro_derive(GraphElem, attributes(graphelem))]
pub fn graph_elem_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("Parsing input failed");
    impl_graph_elem(&ast)
}

fn impl_graph_elem(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = name.to_string();
    let fields = match ast.data {
        syn::Data::Struct(ref data_struct) => &data_struct.fields,
        _ => { panic!("#[derive(GraphElem)] only valid on struct types"); }
    };

    // parse fields into information structure
    let graph_elem_fields = match fields {
        syn::Fields::Named(fields) => {
            fields.named.iter().map(GraphElemField::from_syn_field)
        },
        _ => { panic!("[derive(GraphElem)] only valid for structs with named fields"); }
    }.collect::<Vec<_>>();

    // generate field setter methods (both builder methods and setter methods)
    let field_setters = graph_elem_fields.iter().map(BuildSetters::from);

    // generate body details of CountExistingFields implementation
    let num_non_optional = graph_elem_fields.iter()
        .filter(|field| field.extract_option_type().is_none()).count();
    let existing_fields_adders = graph_elem_fields.iter()
        .map(BuildExistingFieldAdder::from);

    // generate body of serialize implementation
    let field_serializers = graph_elem_fields.iter().map(BuildSerializer::from);

    // put everything together
    let tokens = quote!(
        impl #name { #(#field_setters)* }
        impl CountExistFields for #name {
            fn count_existing_fields(&self) -> usize {
                let mut count = #num_non_optional;
                #(#existing_fields_adders)*
                count
            }
        }
        impl Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
                let mut state = serializer.serialize_struct(#name_str,
                    self.count_existing_fields())?;
                #(#field_serializers)*
                state.end()
            }
        }
    );
    tokens.into()
}
