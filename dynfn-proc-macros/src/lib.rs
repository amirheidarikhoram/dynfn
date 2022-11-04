use dynfn_core::types::*;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn dynfn(_: TokenStream, input: TokenStream) -> TokenStream {
    let default_input = input.clone();
    let inner_func = syn::parse_macro_input!(default_input as syn::ItemFn);

    let item_fn = syn::parse_macro_input!(input as syn::ItemFn);
    let fn_name = item_fn.sig.ident.clone();
    let function = Function {
        name: item_fn.sig.ident.to_string(),
        item: item_fn,
    };

    let impl_assertions = ImplAssertions {
        function: function.clone(),
    };

    let data_structure = DataStructure {
        function: function.clone(),
    };

    let data_extraction = DataExtraction {
        name: data_structure.get_name(),
        function: function.clone(),
    };

    let function_call = FunctionCall {
        function: function.clone(),
    };

    let res = quote! {

        #impl_assertions

        #data_structure

        #[allow(non_camel_case_types)]
        pub struct #fn_name;

        impl Callable<String, String> for #fn_name {
            fn call(data: String) -> std::result::Result<String, ()> {
                use serde_json;

                #inner_func

                #data_extraction

                #function_call
            }
        }
    };

    res.into()
}
