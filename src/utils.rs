use proc_macro_helper::prelude::*;
use syn;

pub fn lit_str_value(source: &syn::Lit) -> Option<String> {
    match source {
        syn::Lit::Str(value) => Some(value.value()),
        _ => None,
    }
}

pub fn target_attributes(source: &Variant) -> impl Iterator<Item = Attribute> + '_ {
    source
        .attributes
        .iter()
        .filter(|x| x.name == "ETS")
        .flat_map(|x| x.sub_nodes.clone())
}
