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

        let assertions = types
            .iter()
            .map(|ty| {
                quote! {
                    assert_impl_all!(#ty: Serialize, Deserialize<'static>);
                }
            })
            .collect::<Vec<proc_macro2::TokenStream>>();

        tokens.extend(assertions);
    }
}
