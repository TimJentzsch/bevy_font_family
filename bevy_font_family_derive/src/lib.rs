mod parse;

use parse::FontFamilyAttr;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Lit, Meta};

/// Automatically implements the `FontFamily` trait.
#[proc_macro_derive(LocalizationFolder, attributes(font))]
pub fn font_family_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_font_family(&ast)
}

/// Implementation of the `FontFamily` derive macro.
fn impl_font_family(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let font_attributes = parse::parse_font_family_attributes(&ast.attrs);

    let gen = quote! {
        impl LocalizationFolder for #name {
        }
    };
    gen.into()
}
