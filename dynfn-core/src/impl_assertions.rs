use std::collections::{HashMap};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::PatType;

use crate::function::Function;

pub struct ImplAssertions {
    pub function: Function,
}

impl ToTokens for ImplAssertions {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut types = self
            .function
            .item
            .sig
            .inputs
            .iter()
            .map(|arg| match arg {
                syn::FnArg::Typed(PatType { ty, .. }) => *ty.clone(),
                _ => panic!("Function arguments must be named"),
            })
            .collect::<Vec<syn::Type>>();

        match &self.function.item.sig.output {
            syn::ReturnType::Default => {}
            syn::ReturnType::Type(_, ty) => types.push(*ty.clone()),
        };

        let mut types_hash_map: HashMap<String, TokenStream> = HashMap::new();

        for ty in types.into_iter() {
            let key = ty.to_token_stream().to_string();
            let value = ty.to_token_stream();

            types_hash_map.entry(key).or_insert(value);
        }

        let assertions = types_hash_map
            .values()
            .into_iter()
            .map(|ty| {
                quote! {
                    assert_impl_all!(#ty: Serialize, Deserialize<'static>);
                }
            })
            .collect::<Vec<proc_macro2::TokenStream>>();

        tokens.extend(assertions);
    }
}
