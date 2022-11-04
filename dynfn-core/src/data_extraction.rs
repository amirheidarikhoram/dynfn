use quote::{quote, ToTokens};
use syn::{Ident, PatType};

use crate::function::Function;

pub struct DataExtraction {
    pub name: String,
    pub function: Function,
}

impl ToTokens for DataExtraction {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let DataExtraction { function, name, .. } = self;

        let struct_ident = Ident::new(name, proc_macro2::Span::call_site());

        let fiedls = function
            .item
            .sig
            .inputs
            .iter()
            .map(|arg| match arg {
                syn::FnArg::Typed(PatType { pat, .. }) => {
                    let ident = (*pat).clone();
                    quote! {
                        #ident,
                    }
                }
                _ => panic!("Function arguments must be named"),
            })
            .collect::<Vec<proc_macro2::TokenStream>>();

        if fiedls.len() != 0 {
            let extractions = quote! {
                let #struct_ident {
                    #(#fiedls)*
                } = serde_json::from_str(&data.unwrap()).expect("An error occured while parsing json");

            };
            tokens.extend(extractions);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_data_extraction() {
        let function = Function {
            name: "test".to_string(),
            item: parse_quote! {
                fn test(arg1: String, arg2: String) -> String {
                    format!("{} {}", arg1, arg2)
                }
            },
        };

        let data_extraction = DataExtraction {
            name: "Test".to_string(),
            function,
        };

        let expected = quote! {
            let Test {
                arg1,
                arg2,
            } = serde_json::from_str(&data).expect("An error occured while parsing json");
        };

        let mut tokens = proc_macro2::TokenStream::new();
        data_extraction.to_tokens(&mut tokens);

        assert_eq!(expected.to_string(), tokens.to_string());
    }
}
