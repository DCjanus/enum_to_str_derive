extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(EnumToStr)]
pub fn enum_to_str(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let name = &ast.ident;
    if let syn::Body::Enum(body) = ast.body {
        let gen = impl_enum_to_str(name, body);
        gen.parse().unwrap()
    } else {
        panic!("Only work for enum");
    }
}

fn impl_enum_to_str(name: &syn::Ident, body: Vec<syn::Variant>) -> quote::Tokens {
    let content = build_content(name, body);
    quote!(
        impl #name{
            fn enum_to_str(&self)->&'static str{
                match *self{
                    #content
                }
            }
        }
    )
}

fn build_content(name: &syn::Ident, body: Vec<syn::Variant>) -> syn::Ident {
    let lines: Vec<String> = body.iter()
        .map(|field| format!("{enum_name}::{field} => \"{field}\"",
                             field = field.ident,
                             enum_name = name))
        .collect();
    syn::Ident::from(lines.join(","))
}