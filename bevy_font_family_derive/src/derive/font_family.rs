use crate::{
    parse::{parse_font_family_attributes, FontFamilyAttr},
    FontDefinition,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// Implementation of the `FontFamily` derive macro.
pub(crate) fn impl_font_family(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let font_attributes = parse_font_family_attributes(&ast.attrs);

    let (roman_font_definitions, italic_font_definitions) = split_font_attributes(font_attributes);

    let roman_token_stream = convert_to_token_stream(roman_font_definitions);
    let italic_token_stream = convert_to_token_stream(italic_font_definitions);

    quote! {
        impl FontFamily for #name {
            fn roman_fonts() -> Vec<FontDefinition> {
                #roman_token_stream
            }

            fn italic_fonts() -> Vec<FontDefinition> {
                #italic_token_stream
            }
        }
    }
}

/// Split the font attributes into roman and italic font attributes.
fn split_font_attributes(
    attribute_lists: Vec<Vec<FontFamilyAttr>>,
) -> (Vec<FontDefinition>, Vec<FontDefinition>) {
    let mut roman_font_attributes = Vec::new();
    let mut italic_font_attributes = Vec::new();

    for attributes in attribute_lists {
        let mut path = None;
        let mut font_weight = None;
        let mut is_italic = false;

        for attr in attributes {
            match attr {
                FontFamilyAttr::Italic(_) => is_italic = true,
                FontFamilyAttr::Path(_, path_lit) => path = Some(path_lit),
                FontFamilyAttr::Weight(_, expr) => font_weight = Some(expr),
            }
        }

        // Set a default weight if none was given
        let font_weight = if let Some(weight) = font_weight {
            quote! {#weight}
        } else {
            quote! {400}
        };

        let font_definition = FontDefinition {
            path: path.expect("No path specified"),
            font_weight,
        };

        if is_italic {
            italic_font_attributes.push(font_definition);
        } else {
            roman_font_attributes.push(font_definition);
        }
    }

    (roman_font_attributes, italic_font_attributes)
}

/// Convert font definitions to a token stream defining a vector of them.
///
/// Example:
///
/// ```ignore
/// vec![
///     FontDefinition::new(#path, #font_weight),
///     FontDefinition::new(#path, #font_weight),
///     FontDefinition::new(#path, #font_weight),
/// ]
/// ```
fn convert_to_token_stream(font_definitions: Vec<FontDefinition>) -> TokenStream {
    let mut definition_stream = TokenStream::new();

    for FontDefinition { path, font_weight } in font_definitions {
        definition_stream = quote! {
            #definition_stream
            FontDefinition::new(#path, #font_weight),
        };
    }

    quote! {
        vec![
            #definition_stream
        ]
    }
}
