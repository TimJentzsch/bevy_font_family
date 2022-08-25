use proc_macro_error::{abort, ResultExt};
use syn::{
    self,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Ident, LitStr, Token,
};

pub fn parse_font_family_attributes(all_attrs: &[Attribute]) -> Vec<Vec<FontFamilyAttr>> {
    all_attrs
        .iter()
        .filter(|attr| attr.path.is_ident("font"))
        .map(|attr| {
            attr.parse_args_with(Punctuated::<FontFamilyAttr, Token![,]>::parse_terminated)
                .unwrap_or_abort()
                .into_iter()
                .collect()
        })
        .collect()
}

#[derive(Clone)]
pub enum FontFamilyAttr {
    // single-identifier attributes
    Italic(Ident),

    // ident = "string literal"
    Path(Ident, LitStr),
}

impl Parse for FontFamilyAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use self::FontFamilyAttr::*;

        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        if input.peek(Token![=]) {
            // `name = value` attributes.
            let assign_token = input.parse::<Token![=]>()?; // skip '='

            if input.peek(LitStr) {
                let lit: LitStr = input.parse()?;

                match &*name_str {
                    "path" => Ok(Path(name, lit)),
                    _ => abort!(name, "unexpected attribute: {}", name_str),
                }
            } else {
                abort! {
                    assign_token,
                    "expected `string literal`after `=`"
                }
            }
        } else {
            // Attributes represented with a sole identifier.
            match name_str.as_ref() {
                "italic" => Ok(Italic(name)),
                _ => abort!(name, "unexpected attribute: {}", name_str),
            }
        }
    }
}
