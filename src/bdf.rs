use std::{
    collections::HashMap,
};

//

use crate::{
    bitmap::Bitmap,
};

//

pub enum WritingMetrics {
    Normal = 0,
    Alternate,
    Both,
}

// #[derive(Debug)]
pub struct Glyph {
    pub name: String,
    pub codepoint: char,
    pub bounding_box: BoundingBox,
    pub bitmap: Bitmap,

    pub metrics: WritingMetrics,

    pub scalable_width: Option<(u32, u32)>,
    pub device_width: Option<(u32, u32)>,
    pub scalable_width_alt: Option<(u32, u32)>,
    pub device_width_alt: Option<(u32, u32)>,

    pub vector: Option<(u32, u32)>,

}

impl Glyph {
    pub fn new(name: &str, codepoint: char, bounding_box: BoundingBox, bitmap: Bitmap) -> Self {
        Self {
            name: String::from(name),
            codepoint,
            bounding_box,
            metrics: WritingMetrics::Normal,
            bitmap,

            scalable_width: None,
            device_width: None,
            scalable_width_alt: None,
            device_width_alt: None,
            vector: None,
        }
    }

    pub fn validate(&self) -> bool {
        match self.metrics {
            WritingMetrics::Normal => {
                self.scalable_width_alt.is_none() &&
                self.device_width_alt.is_none()
            }
            _ => {
                self.scalable_width_alt.is_some() &&
                self.device_width_alt.is_some()
            }
        }
    }
}

// #[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct BoundingBox {
    pub width: u32,
    pub height: u32,
    pub x_offset: i32,
    pub y_offset: i32,
}

// #[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct FontSize {
    pub point_sz: u16,
    pub x_dpi: u16,
    pub y_dpi: u16,
}

pub enum Property {
    Str(String),
    Int(i32),
}

// #[derive(Debug)]
pub struct Font {
    pub bdf_version: String,
    pub name: String,
    pub size: FontSize,
    pub bounding_box: BoundingBox,
    pub metrics: WritingMetrics,

    pub properties: HashMap<String, Property>,
    pub glyphs: HashMap<char, Glyph>,

    pub content_version: Option<String>,
    pub scalable_width: Option<(u32, u32)>,
    pub device_width: Option<(u32, u32)>,
    pub scalable_width_alt: Option<(u32, u32)>,
    pub device_width_alt: Option<(u32, u32)>,
    pub vector: Option<(u32, u32)>,

}

impl Font {
    pub fn new(name: &str, size: FontSize, bounding_box: BoundingBox) -> Self {
        Self {
            bdf_version: String::from("2.2"),
            name: String::from(name),
            size,
            bounding_box,
            metrics: WritingMetrics::Normal,

            properties: HashMap::new(),
            glyphs: HashMap::new(),

            content_version: None,
            scalable_width: None,
            device_width: None,
            scalable_width_alt: None,
            device_width_alt: None,
            vector: None,
        }
    }

    pub fn validate(&self) -> bool {
        // TODO if d width not defined for font, must be defined for all glyphs
        //   d width alt must also be defined for metric::alt
        // validate all glyphs
        //   GlyphInvalid(glyph) error
        true
    }
}
