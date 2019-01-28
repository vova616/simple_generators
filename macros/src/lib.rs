#![feature(generators, generator_trait)]

extern crate proc_macro;

use self::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn generator(_meta: TokenStream, input: TokenStream) -> TokenStream {
    let mut input: syn::ItemFn = syn::parse(input).unwrap();
    let block = input.block;
    input.block = syn::parse(
        (quote!{{
            self::GeneratorIteratorAdapter({
                move || { #block }
            })
        }})
        .into(),
    )
    .unwrap();

    let output = quote!{  #input  };
    output.into()
}
