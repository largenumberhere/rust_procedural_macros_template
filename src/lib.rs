extern crate proc_macro;
use quote::quote;

type TokenStream1 = proc_macro::TokenStream;
type TokenStream2 = proc_macro2::token_stream::TokenStream;

type TokenTree1 = proc_macro::TokenTree;
type TokenTree2 = proc_macro2::TokenTree;

#[proc_macro]
pub fn hello_fn_proc_macro(input: TokenStream1) -> TokenStream1 {
    let output: TokenStream2 = quote!{
        {
            println!("Hello world!");
            "Hello world"
        }
    };


    let output: TokenStream1 = output.into();
    output
}