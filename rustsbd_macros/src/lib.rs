extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Process)]
pub fn process_macro_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    impl_process_macro(&input)
}

fn impl_process_macro(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let gen = quote! {
        impl Process for #name {
            fn rules_vec(&self) -> Vec<Rule> {
                self.rules.to_vec()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(FmtDisplay)]
pub fn display_macro_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    impl_display_macro(&input)
}

fn impl_display_macro(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let gen = quote! {
        impl FmtDisplay for #name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self.rules)
            }
        }
    };
    gen.into()
}
