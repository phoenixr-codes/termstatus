#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote};

/// Turns an `enum` into an Terminal Status.
///
/// This will implement [Display](std::fmt::Display) which pads the name of the
/// variant by prefixing it with spaces so that each label ends at the same
/// column in the terminal.
///
/// The name of the label can be changed by giving the `enum` variant an
/// attribute of the form `#[display = "Other Name"]`. This will override
/// the label when displaying it.
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
///     #[display = "Cleaning Up"]
///     CleaningUp,
///     Finished,
///     Running,
/// }
///
/// fn main() {
///     println!("{} foo", Status::Building);
///     println!("{} foo", Status::Built);
///     println!("{} bar", Status::Running);
///     println!("{} bar", Status::Finished);
///     println!("{} baz", Status::CleaningUp);
/// }
/// ```
///
/// Running this will display the following in the terminal:
///
/// ```text
///    Building foo
///       Built foo
///     Running bar
///    Finished bar
/// Cleaning Up baz
/// ```
#[proc_macro_derive(TermStatus, attributes(display, style))]
pub fn generate_format(input: TokenStream) -> TokenStream {
    // Get the `enum` input as a token stream
    let input = parse_macro_input!(input as syn::ItemEnum);

    // Save the name of the enum
    let ident = input.ident.clone();

    // Calculate the length of the enum variant with the longest name
    // and create a match arm for each variant that binds it to the
    // string.
    let mut max_len: usize = 0;
    let mut match_arms: Vec<syn::Arm> = Vec::new();
    for variant in &input.variants {
        let name = variant.ident.to_string();
        let mut display = name.clone();

        // Parse attributes of variant
        for attr in &variant.attrs {
            match attr.style {
                syn::AttrStyle::Inner(_) => panic!("Expeced outer attribute, got inner attribute"),
                syn::AttrStyle::Outer => match &attr.meta {
                    syn::Meta::Path(_) => panic!(
                        r#"Expected attribute of form `#[style(blue, bold)]` or `#[display = "Other Label"]`, got one of form `#[unknown]`"#
                    ),
                    syn::Meta::List(l) => {
                        // #[style(red, bold)]
                        todo!();
                    }
                    syn::Meta::NameValue(nv) => {
                        // #[display = "Other Label"]
                        if nv.path.leading_colon.is_some() {
                            panic!("Unexpected leading `::`");
                        };

                        let attr_name = nv.path.segments.first().unwrap().ident.to_string();
                        if nv.path.segments.len() > 1 || attr_name != "display" {
                            panic!("Expected 'display', got '{}'", attr_name);
                        };

                        match &nv.value {
                            syn::Expr::Lit(syn::ExprLit {
                                attrs: _,
                                lit: syn::Lit::Str(s),
                            }) => {
                                display = s.value();
                            }
                            _ => panic!("Expected string literal"),
                        };
                    }
                },
            };
        }

        let length = display.len();
        if length > max_len {
            max_len = length;
        };

        let variant_ident = &variant.ident;
        // TODO: can we use quote::quote instead?
        match_arms.push(parse_quote! {
            #ident::#variant_ident => #display
        })
    }

    // implement Display
    quote! {
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let name = match self { #(#match_arms),* };
                let pad = " ".repeat(#max_len - name.len());

                let mut padded = std::string::String::new();
                padded.push_str(pad.as_str());
                padded.push_str(name);

                write!(f, "{}", padded)
            }
        }
    }
    .into()
}
