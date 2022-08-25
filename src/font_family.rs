use std::marker::PhantomData;

use crate::{font_weights::*, FontBuilder, FontDefinition};

pub trait FontFamily: Sized {
    /// The roman (non-italic) fonts of the font family.
    ///
    /// - The fonts MUST be sorted by `font_weight`, ascending.
    fn roman_fonts() -> Vec<FontDefinition>;

    /// The italic fonts of the font family.
    ///
    /// - The fonts MUST be sorted by `font_weight`, ascending.
    fn italic_fonts() -> Vec<FontDefinition>;

    /// Build a font of this font family.
    fn font() -> FontBuilder<Self> {
        FontBuilder {
            font_family: PhantomData,
            is_italic: false,
            font_weight: REGULAR,
        }
    }
}
