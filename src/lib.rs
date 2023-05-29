#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, ItemEnum};

/// Turns an `enum` into an Terminal Status.
///
/// This will implement [Display](std::fmt::Display) which pads the name of the
/// variant by prefixing it with spaces so that each label ends at the same
/// column in the terminal.
///
/// # Example
///
/// ```rust
/// use termstatus::TermStatus;
///
/// #[derive(TermStatus)]
/// pub enum Status {
///     Building,
///     Built,
///     Finished,
///     Running,
/// }
///
/// fn main() {
///     println!("{} foo", Status::Building);
///     println!("{} foo", Status::Built);
///     println!("{} bar", Status::Running);
///     println!("{} bar", Status::Finished);
/// }
/// ```
///
/// Running this will display the following in the terminal:
///
/// ```text
/// Building foo
///    Built foo
///  Running bar
/// Finished bar
/// ```
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
