#![deny(warnings)]
extern crate proc_macro;
extern crate proc_macro_helper;
#[macro_use]
extern crate quote;
extern crate syn;

mod utils;

use proc_macro::TokenStream;
use proc_macro_helper::prelude::*;
use syn::{DeriveInput, Ident};
use utils::*;

#[proc_macro_derive(EnumToStr, attributes(ETS))]
pub fn enum_to_str(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse::<DeriveInput>(input).unwrap();
    let target_enum = Enum::parse(&derive_input);
    let enum_name = syn::parse_str::<Ident>(target_enum.name.as_ref()).unwrap();
    let match_content = target_enum
        .variants
        .iter()
        .map(|x| each_variant(x, &enum_name));

    quote!(
        impl #enum_name {
            pub fn enum_to_str(&self) -> &'static str {
                match *self {
                    #(#match_content),*
                }
            }
        }
    ).into()
}

fn each_variant(source: &Variant, enum_name: &Ident) -> impl quote::ToTokens {
    let variant_name: Ident = syn::parse_str(source.name.as_ref()).unwrap();
    let variant_content = if source.fields.is_empty() {
        quote!()
    } else {
        let temp = vec![quote!(_); source.fields.len()];
        quote!((#(#temp),*))
    };
    let variant_value = variant_value(source);
    quote!( #enum_name::#variant_name#variant_content => #variant_value)
}

fn variant_value(source: &Variant) -> impl quote::ToTokens {
    if let Some(value_attribute) = target_attributes(source).find(|x| x.name == "value") {
        let value = lit_str_value(
            value_attribute
                .values
                .first()
                .expect("string value must be set"),
        ).expect("value must be string");
        return quote!(#value);
    }

    let value = source.name.clone();
    return quote!(#value);
}
