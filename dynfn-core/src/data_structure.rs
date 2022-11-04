use quote::{quote, ToTokens};
use syn::PatType;

use crate::function::Function;

pub struct DataStructure {
    pub function: Function,
}

impl DataStructure {
    pub fn get_name(&self) -> String {
        format!("{}{}", self.function.name[0..1].to_uppercase() + &self.function.name[1..], "Data")
    }
}

impl ToTokens for DataStructure {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let struct_ident = syn::Ident::new(
            &self.get_name().as_str(),
            proc_macro2::Span::call_site(),
        );

        let fiedls = self
            .function
            .item
            .sig
            .inputs
            .iter()
            .map(|arg| match arg {
                syn::FnArg::Typed(PatType { pat, ty, .. }) => {
                    let ident = (*pat).clone();
                    let ty = (*ty).clone();
                    quote! {
                        #ident: #ty,
                    }
                }
                _ => panic!("Function arguments must be named"),
            })
            .collect::<Vec<proc_macro2::TokenStream>>();
        let data_struct = quote! {
            #[derive(Serialize, Deserialize)]
            struct #struct_ident {
                #(#fiedls)*
            }
        };

        tokens.extend(data_struct);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_data_structure() {
        let function = Function {
            name: "test".to_string(),
            item: parse_quote! {
                fn test(arg1: String, arg2: String) -> String {
                    format!("{} {}", arg1, arg2)
                }
            },
        };

        let data_structure = DataStructure { function };

        let expected = quote! {
            #[derive(serde::Serialize, serde::Deserialize)]
            struct TestData {
                arg1: String,
                arg2: String,
            }
        };

        let mut tokens = proc_macro2::TokenStream::new();
        data_structure.to_tokens(&mut tokens);

        assert_eq!(expected.to_string(), tokens.to_string());
    }
}