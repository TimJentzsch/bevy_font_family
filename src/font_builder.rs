use std::path::PathBuf;

use bevy::asset::AssetPath;

use crate::{font_weights::*, FontFamily};

#[derive(Debug, Clone)]
pub struct FontBuilder<'a, F: FontFamily> {
    /// The font family that this builder is for.
    pub(crate) font_family: &'a F,

    /// Whether the font should be italic.
    pub(crate) is_italic: bool,

    /// The target font weight.
    pub(crate) font_weight: u16,
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

impl<'a, 'f, F: FontFamily> From<FontBuilder<'f, F>> for AssetPath<'a> {
    fn from(font_builder: FontBuilder<'f, F>) -> Self {
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

impl<'a, 'f, F: FontFamily> From<&'a mut FontBuilder<'f, F>> for AssetPath<'a> {
    fn from(font_builder: &'a mut FontBuilder<'f, F>) -> Self {
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
