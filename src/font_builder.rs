use std::{marker::PhantomData, path::PathBuf};

use bevy::asset::AssetPath;

use crate::{font_weights::*, FontDefinition, FontFamily};

#[derive(Debug, Clone)]
pub struct FontBuilder<F: FontFamily> {
    /// The font family that this builder is for.
    pub(crate) font_family: PhantomData<F>,

    /// Whether the font should be italic.
    pub(crate) is_italic: bool,

    /// The target font weight.
    pub(crate) font_weight: u16,
}

impl<F: FontFamily> FontBuilder<F> {
    fn select_closest_font_weight(
        fonts: &[FontDefinition],
        font_weight: u16,
    ) -> Option<&FontDefinition> {
        fonts
            .iter()
            .min_by_key(|def| ((font_weight as i32) - (def.font_weight as i32)).abs())
    }

    /// Get the path to the font, based on the selected attributes.
    pub fn path(&self) -> String {
        if self.is_italic {
            Self::select_closest_font_weight(&F::italic_fonts(), self.font_weight)
                .unwrap()
                .path
                .clone()
        } else {
            Self::select_closest_font_weight(&F::roman_fonts(), self.font_weight)
                .unwrap()
                .path
                .clone()
        }
    }

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

impl<'a, F: FontFamily> From<FontBuilder<F>> for AssetPath<'a> {
    fn from(font_builder: FontBuilder<F>) -> Self {
        let path = font_builder.path();
        AssetPath::new(PathBuf::from(path), None)
    }
}

impl<'a, F: FontFamily> From<&'a mut FontBuilder<F>> for AssetPath<'a> {
    fn from(font_builder: &'a mut FontBuilder<F>) -> Self {
        let path = font_builder.path();
        AssetPath::new(PathBuf::from(path), None)
    }
}
