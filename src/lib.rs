// TODO: support #[display("foo")] attribute on variant
// TODO: support #[color("green")] attribute on variant

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, ItemEnum};

#[proc_macro_derive(TermStatus)]
pub fn generate_format(input: TokenStream) -> TokenStream {
    // Get the `enum` input as a token stream
    let input = parse_macro_input!(input as ItemEnum);

    // Save the name of the enum
    let ident = input.ident.clone();

    // Calculate the length of the enum variant with the longest name
    // and create a match arm for each variant that binds it to the
    // string.
    let mut max_len: usize = 0;
    let mut match_arms: Vec<syn::Arm> = Vec::new();
    for variant in &input.variants {
        let name = variant.ident.to_string();
        let length = name.len();
        if length > max_len {
            max_len = length;
        };

        let variant_ident = variant.ident.clone();
        match_arms.push(parse_quote! {
            #ident::#variant => stringify!(#variant_ident)
        })
    }

    // implement Display
    quote! {
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let name = match self { #(#match_arms),* };
                let pad = " ".repeat(#max_len - name.len());

                let mut padded = String::new();
                padded.push_str(pad.as_str());
                padded.push_str(name);

                write!(f, "{}", padded)
            }
        }
    }
    .into()
}
