use syn;
use quote::{ToTokens};
use proc_macro2 as pm2;

#[derive(Clone)]
pub struct GraphElemField {
    name: syn::Ident,
    setter_name: syn::Ident,
    ty: syn::Type,
    attrs: GraphElemAttrs
}
impl GraphElemField {
    /// Parse a syn `Field` object.
    pub fn from_syn_field(field: &syn::Field) -> GraphElemField {
        let attrs = GraphElemAttrs::from_syn_attrs(&field.attrs);
        let field_name = field.ident.clone().expect("unnamed field in FieldNames struct");
        GraphElemField {
            setter_name: syn::Ident::new(&format!("set_{}", field_name), pm2::Span::call_site()),
            name: field_name,
            ty: field.ty.clone(),
            attrs
        }
    }
    pub fn extract_option_type<'a>(&'a self) -> Option<&'a syn::Type> {
        match self.ty {
            syn::Type::Path(syn::TypePath { ref path, .. }) => {
                let relative_ty = path.segments.iter().last().expect("empty path");
                if relative_ty.ident.to_string() == "Option" {
                    match &relative_ty.arguments {
                        syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                            args, ..
                        }) => {
                            match args.iter().last().expect("option without generic parameters") {
                                syn::GenericArgument::Type(ty) => Some(ty),
                                _ => None
                            }
                        },
                        _ => None
                    }
                } else {
                    None
                }
            }
            _ => None
        }
    }
}

#[derive(Clone, Default)]
struct GraphElemAttrs {
    serialize_rename: Option<String>,
}
impl GraphElemAttrs {
    fn from_syn_attrs(attrs: &Vec<syn::Attribute>) -> GraphElemAttrs {
        const GRAPHELEM_ATTR_IDENT: &str = "graphelem";
        const SERIALIZE_SUBATTR_IDENT: &str = "serialize";
        let mut gea = GraphElemAttrs::default();

        for attr in attrs.iter() {
            // ignore inner-style (#![...]) attributes
            if let syn::AttrStyle::Inner(_) = attr.style {
                continue;
            }
            match attr.interpret_meta() {
                Some(syn::Meta::List(meta_list)) => {
                    // only handle 'graphelem' attributes
                    if meta_list.ident.to_string().as_str() == GRAPHELEM_ATTR_IDENT {
                        for meta in &meta_list.nested {
                            match meta {
                                syn::NestedMeta::Meta(syn::Meta::List(nested_list)) => {
                                    match nested_list.ident.to_string().as_str() {
                                        SERIALIZE_SUBATTR_IDENT => {
                                            parse_serialize_attrs(&mut gea,
                                                nested_list.nested.iter());
                                        },
                                        _ => { continue; }
                                    }
                                },
                                _ => {
                                    continue;
                                }
                            }
                        }
                    }
                },
                _ => { continue; } // ignore other types of meta-attributes
            }
        }
        gea
    }
}

fn parse_serialize_attrs<'a, T>(gea: &mut GraphElemAttrs, nested: T)
    where T: Iterator<Item=&'a syn::NestedMeta>
{
    const RENAME_IDENT: &str = "name";
    for meta in nested {
        match meta {
            syn::NestedMeta::Meta(syn::Meta::NameValue(mnv)) => {
                if mnv.ident.to_string().as_str() == RENAME_IDENT {
                    match mnv.lit {
                        syn::Lit::Str(ref litstr) => {
                            gea.serialize_rename = Some(litstr.value().clone());
                        },
                        _ => {
                            panic!("serialize renaming only accepts string types");
                        }
                    }
                }
            },
            _ => { continue; }
        }
    }
}

pub struct BuildSetters<'a>(&'a GraphElemField);
impl<'a> From<&'a GraphElemField> for BuildSetters<'a> {
    fn from(other: &'a GraphElemField) -> BuildSetters<'a> { BuildSetters(other) }
}

impl<'a> ToTokens for BuildSetters<'a> {
    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
        let GraphElemField { ref name, ref setter_name, ref ty, .. } = self.0;
        let (value_assigned, ty) = if let Some(inner_ty) = self.0.extract_option_type() {
            (quote!(Some(value.into())), inner_ty)
        } else {
            (quote!(value.into()), ty)
        };
        quote!(
            pub fn #name<T: Into<#ty>>(
                mut self,
                value: T
            ) -> Self {
                self.#name = #value_assigned;
                self
            }
            pub fn #setter_name<T: Into<#ty>>(
                &mut self,
                value: T
            ) {
                self.#name = #value_assigned;
            }
        ).to_tokens(tokens);
    }
}

pub struct BuildExistingFieldAdder<'a>(&'a GraphElemField);
impl<'a> From<&'a GraphElemField> for BuildExistingFieldAdder<'a> {
    fn from(other: &'a GraphElemField) -> BuildExistingFieldAdder<'a> {
        BuildExistingFieldAdder(other)
    }
}

impl<'a> ToTokens for BuildExistingFieldAdder<'a> {
    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
        let GraphElemField { ref name, .. } = self.0;
        if self.0.extract_option_type().is_some() {
            quote!( if self.#name.is_some() { count += 1; } ).to_tokens(tokens);
        }
    }
}

pub struct BuildSerializer<'a>(&'a GraphElemField);
impl<'a> From<&'a GraphElemField> for BuildSerializer<'a> {
    fn from(other: &'a GraphElemField) -> BuildSerializer<'a> {
        BuildSerializer(other)
    }
}

impl<'a> ToTokens for BuildSerializer<'a> {
    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
        let GraphElemField { ref name, ref attrs, .. } = self.0;
        let name_str = match attrs.serialize_rename {
            Some(ref renamed) => renamed.clone(),
            None => name.to_string()
        };
        if self.0.extract_option_type().is_some() {
            quote!(
                if let Some(ref value) = self.#name {
                    state.serialize_field(#name_str, &value)?;
                };
            ).to_tokens(tokens);
        } else {
            quote!(
                state.serialize_field(#name_str, &value)?;
            ).to_tokens(tokens);
        }
    }
}
