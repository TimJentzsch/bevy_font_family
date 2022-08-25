use crate::{font_weights::*, FontBuilder, FontDefinition};

pub trait FontFamily: Sized {
    /// The roman (non-italic) fonts of the font family.
    ///
    /// - The fonts MUST be sorted by `font_weight`, ascending.
    fn roman_fonts(&self) -> Vec<FontDefinition>;

    /// The italic fonts of the font family.
    ///
    /// - The fonts MUST be sorted by `font_weight`, ascending.
    fn italic_fonts(&self) -> Vec<FontDefinition>;

    fn thin(&self) -> FontBuilder<Self> {
        FontBuilder {
            font_family: self,
            is_italic: false,
            font_weight: THIN,
        }
    }

    fn light(&self) -> FontBuilder<Self> {
        FontBuilder {
            font_family: self,
            is_italic: false,
            font_weight: LIGHT,
        }
    }

    fn regular(&self) -> FontBuilder<Self> {
        FontBuilder {
            font_family: self,
            is_italic: false,
            font_weight: REGULAR,
        }
    }

    fn medium(&self) -> FontBuilder<Self> {
        FontBuilder {
            font_family: self,
            is_italic: false,
            font_weight: MEDIUM,
        }
    }

    fn bold(&self) -> FontBuilder<Self> {
        FontBuilder {
            font_family: self,
            is_italic: false,
            font_weight: BOLD,
        }
    }

    fn black(&self) -> FontBuilder<Self> {
        FontBuilder {
            font_family: self,
            is_italic: false,
            font_weight: BLACK,
        }
    }

    fn italic(&self) -> FontBuilder<Self> {
        FontBuilder {
            font_family: self,
            is_italic: true,
            font_weight: REGULAR,
        }
    }

    fn roman(&self) -> FontBuilder<Self> {
        FontBuilder {
            font_family: self,
            is_italic: false,
            font_weight: REGULAR,
        }
    }
}
