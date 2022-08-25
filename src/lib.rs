use std::path::PathBuf;

use bevy::asset::AssetPath;

const THIN: u16 = 100;
const LIGHT: u16 = 300;
const REGULAR: u16 = 400;
const MEDIUM: u16 = 500;
const BOLD: u16 = 700;
const BLACK: u16 = 900;

pub struct FontDefinition {
    /// The path to the font file, relative to the asset folder.
    path: String,

    /// The weight of the font.
    ///
    /// Common values:
    ///
    /// - 100: Thin
    /// - 300: Light
    /// - 400: Regular (Default)
    /// - 500: Medium
    /// - 700: Bold
    /// - 900: Black
    font_weight: u16,
}

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

pub struct FontBuilder<'a, F: FontFamily> {
    /// The font family that this builder is for.
    font_family: &'a F,

    /// Whether the font should be italic.
    is_italic: bool,

    /// The target font weight.
    font_weight: u16,
}

impl<'a, F: FontFamily> FontBuilder<'a, F> {
    pub fn thin(&mut self) -> &mut Self {
        self.font_weight = THIN;
        self
    }

    pub fn light(&mut self) -> &mut Self {
        self.font_weight = LIGHT;
        self
    }

    pub fn regular(&mut self) -> &mut Self {
        self.font_weight = REGULAR;
        self
    }

    pub fn medium(&mut self) -> &mut Self {
        self.font_weight = MEDIUM;
        self
    }

    pub fn bold(&mut self) -> &mut Self {
        self.font_weight = BOLD;
        self
    }

    pub fn black(&mut self) -> &mut Self {
        self.font_weight = BLACK;
        self
    }

    pub fn italic(&mut self) -> &mut Self {
        self.is_italic = true;
        self
    }

    pub fn roman(&mut self) -> &mut Self {
        self.is_italic = false;
        self
    }
}

impl<'a, 'f, F: FontFamily> From<&'a FontBuilder<'f, F>> for AssetPath<'a> {
    fn from(font_builder: &'a FontBuilder<'f, F>) -> Self {
        let path = if font_builder.is_italic {
            font_builder
                .font_family
                .italic_fonts()
                // TODO: Search for the best matching font weight
                .first()
                .unwrap()
                .path
                .clone()
        } else {
            font_builder
                .font_family
                .roman_fonts()
                // TODO: Search for the best matching font weight
                .first()
                .unwrap()
                .path
                .clone()
        };

        AssetPath::new(PathBuf::from(path), None)
    }
}
