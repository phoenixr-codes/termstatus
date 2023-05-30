#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, punctuated::Punctuated};

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
/// The style such as the color may be modified with an attribute of the form
/// `#[style(red, on_yellow)]`.
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
///     #[style(green, bold)]
///     Finished,
///     #[style(on_green, bold)]
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
/// Running this will display the following in the terminal (without styles
/// because this is a Markdown document, not a terminal):
///
/// ```text
///    Building foo
///       Built foo
///     Running bar
///    Finished bar
/// Cleaning Up baz
/// ```
///
/// # Available Style Identifiers
///
/// - `on_black`
/// - `on_red`
/// - `on_green`
/// - `on_yellow`
/// - `on_blue`
/// - `on_magenta`
/// - `on_cyan`
/// - `on_white`
/// - `black`
/// - `red`
/// - `green`
/// - `yellow`
/// - `blue`
/// - `magenta`
/// - `cyan`
/// - `white`
/// - `bold`
/// - `dim`
/// - `italic`
/// - `underlined`
/// - `blink`
/// - `blinkfast`
/// - `reverse`
/// - `hidden`
/// - `strikethrough`
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
        let mut garbage = 0; // amounnt of invisible characters

        // Parse attributes of variant
        for attr in &variant.attrs {
            match attr.style {
                syn::AttrStyle::Inner(_) => panic!("Expeced outer attribute, got inner attribute"),
                syn::AttrStyle::Outer => match &attr.meta {
                    syn::Meta::Path(_) => panic!(
                        r#"Expected attribute of form `#[style(blue, bold)]` or `#[display = "Other Label"]`, got one of form `#[unknown]`"#
                    ),
                    // TODO: it might be possible to remove checks, due to the
                    //       `attributes` argument in the function attribute
                    // TODO: put checks together
                    syn::Meta::List(l) => {
                        // #[style(red, bold)]
                        if l.path.leading_colon.is_some() {
                            panic!("Unexpected leading `::`");
                        };

                        let attr_name = l.path.segments.first().unwrap().ident.to_string();
                        if l.path.segments.len() > 1 || attr_name != "style" {
                            panic!("Expected 'style', got '{}'", attr_name);
                        };

                        let args: Punctuated<syn::Ident, syn::Token![,]> = l
                            .parse_args_with(Punctuated::parse_terminated)
                            .expect("Invalid attribute syntax");

                        for arg in args {
                            let style_name = arg.to_string();
                            // TODO: we can do better
                            let ansi = match style_name.as_str() {
                                "black" => "\x1b[30m",
                                "red" => "\x1b[31m",
                                "green" => "\x1b[32m",
                                "yellow" => "\x1b[33m",
                                "blue" => "\x1b[34m",
                                "magenta" => "\x1b[35m",
                                "cyan" => "\x1b[36m",
                                "white" => "\x1b[37m",

                                "on_black" => "\x1b[40m",
                                "on_red" => "\x1b[41m",
                                "on_green" => "\x1b[42m",
                                "on_yellow" => "\x1b[43m",
                                "on_blue" => "\x1b[44m",
                                "on_magenta" => "\x1b[45m",
                                "on_cyan" => "\x1b[46m",
                                "on_white" => "\x1b[47m",

                                "bold" => "\x1b[1m",
                                "dim" => "\x1b[2m",
                                "italic" => "\x1b[3m",
                                "underlined" => "\x1b[4m",
                                "blink" => "\x1b[5m",
                                "blinkfast" => "\x1b[6m",
                                "reverse" => "\x1b[7m",
                                "hidden" => "\x1b[8m",
                                "strikethrough" => "\x1b[9m",

                                &_ => panic("Unknown style identifier"),
                            };
                            let mut styled = String::from(ansi);
                            let reset = "\x1b[0m";
                            garbage += styled.chars().count() + reset.chars().count();
                            styled.push_str(&display);
                            styled.push_str(reset);
                            display = styled;
                        }
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

        let length = display.chars().count() - garbage;
        if length > max_len {
            max_len = length;
        };

        let variant_ident = &variant.ident;
        // TODO: can we use quote::quote instead?
        match_arms.push(parse_quote! {
            #ident::#variant_ident => (#display, #length)
        })
    }

    // implement Display
    quote! {
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let (name, real_len) = match self { #(#match_arms),* };
                let pad = " ".repeat(#max_len - real_len);

                let mut padded = std::string::String::new();
                padded.push_str(pad.as_str());
                padded.push_str(name);

                write!(f, "{}", padded)
            }
        }
    }
    .into()
}
