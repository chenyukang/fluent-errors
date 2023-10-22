use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use syn::LitStr;
use syn::{parse_macro_input, ItemStruct};

fn hash_string(input: &str) -> String {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let hash_value = hasher.finish();
    format!("{:07x}", hash_value % 0x7fffffff)
}

#[proc_macro_attribute]
pub fn diag(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args_clone = attr.clone();

    let mut label: Option<LitStr> = None;
    let mut msg: Option<LitStr> = None;
    let args_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("label") {
            label = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("msg") {
            msg = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            let value = meta.value();
            eprintln!("value: {:?}", value);
            Err(meta.error("unsupported property"))
        }
    });

    parse_macro_input!(args_clone with args_parser);
    if let Some(msg) = msg {
        eprintln!("msg: {}", msg.value());
    }
    if let Some(label) = label {
        eprintln!("label: {}", label.value());
    }

    let attr_str = attr.to_string();
    eprintln!("attr_str: {}", attr_str);
    let item_struct = parse_macro_input!(item as ItemStruct);
    let vis = &item_struct.vis;
    let struct_name = &item_struct.ident;
    let gen_fn_name = format_ident!("gen_{}", struct_name.to_string().to_case(Case::Snake));
    let hash = hash_string(&attr_str);
    let fluent_hash_tag = format!("{}-{}", struct_name.to_string().to_case(Case::Kebab), hash);
    quote!(
        #item_struct

        #vis fn #gen_fn_name() -> (String, String) {
            (#fluent_hash_tag.to_string(), #attr_str.to_string())
        }
    )
    .into()
}
