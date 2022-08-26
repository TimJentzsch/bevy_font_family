mod derive;
mod parse;

use derive::font_family::impl_font_family;
use proc_macro::TokenStream;
use syn::LitStr;

#[derive(Clone)]
struct FontDefinition {
    pub path: LitStr,
    pub font_weight: u16,
}

/// Automatically implements the `FontFamily` trait.
#[proc_macro_derive(LocalizationFolder, attributes(font))]
pub fn font_family_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_font_family(&ast).into()
}
