use quote::{quote, ToTokens};
use syn::PatType;

use super::Function;

pub struct FunctionCall {
    pub function: Function,
}

impl ToTokens for FunctionCall {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let item_fn = (&self.function.item).clone();

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
            let result = #func(#(#arguments),*).await;
            serde_json::to_string(&result).expect("An error occured while parsing json")
        };

        tokens.extend(function_call);
    }
}

#[cfg(test)]
mod tests {
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
            serde_json::to_string(&result).expect("An error occured while parsing json")
        };

        let mut tokens = proc_macro2::TokenStream::new();
        function_call.to_tokens(&mut tokens);

        assert_eq!(expected.to_string(), tokens.to_string());
    }
}