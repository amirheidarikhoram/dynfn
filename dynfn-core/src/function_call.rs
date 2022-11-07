use quote::{quote, ToTokens};
use syn::PatType;

use crate::function::Function;

pub struct FunctionCall {
    pub function: Function,
}

impl ToTokens for FunctionCall {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let item_fn = (&self.function.item).clone();

        let asyncness = item_fn.sig.asyncness.clone();

        let await_execution = match asyncness {
            Some(_) => quote::quote!(
                .await;
            ),
            None => quote::quote!(;)
        };

        let func = (&item_fn.sig.ident).clone();

        let arguments = item_fn.sig.inputs.iter().map(|arg| match arg {
            syn::FnArg::Typed(PatType { pat, .. }) => {
                let ident = (*pat).clone();
                quote! {
                    #ident
                }
            }
            _ => panic!("Function arguments must be named"),
        });

        let function_call = quote! {
            let result = #func(#(#arguments),*)#await_execution
            match serde_json::to_string(&result) {
                Ok(ser_result) => {
                    Ok(ser_result)
                }
                Err(_) => {
                    Err(())
                }
            }
        };

        tokens.extend(function_call);
    }
}

#[cfg(test)]
mod function_call_tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_function_call() {
        let function = Function {
            name: "test".to_string(),
            item: parse_quote! {
                async fn test(arg1: String, arg2: String) -> String {
                    format!("{} {}", arg1, arg2)
                }
            },
        };

        let function_call = FunctionCall { function };

        let expected = quote! {
            let result = test(arg1, arg2).await;
            match serde_json::to_string(&result) {
                Ok(ser_result) => {
                    Ok(ser_result)
                }
                Err(_) => {
                    Err(())
                }
            }
        };

        let mut tokens = proc_macro2::TokenStream::new();
        function_call.to_tokens(&mut tokens);

        assert_eq!(expected.to_string(), tokens.to_string());
    }
}