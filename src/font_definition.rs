#[derive(Debug, Clone)]
pub struct FontDefinition {
    /// The path to the font file, relative to the asset folder.
    pub path: String,

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
    pub font_weight: u16,
}

impl FontDefinition {
    pub fn new<P>(path: P, font_weight: u16) -> Self
    where
        P: Into<String>,
    {
        Self {
            path: path.into(),
            font_weight,
        }
    }
}
