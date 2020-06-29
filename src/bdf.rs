use std::fmt;

use bit_vec::BitVec;

//

pub struct ForBdf<'a, T>(&'a T);

pub trait BdfElement: Sized {
    fn for_bdf(&self) -> ForBdf<Self> {
        ForBdf(self)
    }
}

//

// TODO error type for reading and invalid fonts/chars

#[derive(Clone)]
pub struct Bitmap {
    width: usize,
    height: usize,
    data: Vec<BitVec>
}

impl Bitmap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![BitVec::from_elem(width, false); height]
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn rows(&self) -> &[BitVec] {
        &self.data
    }

    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.data[x][y])
        }
    }

    pub fn set(&mut self, x: usize, y: usize, to: bool) {
        if x >= self.width || y >= self.height {
            return;
        } else {
            self.data[x].set(y, to);
        }
    }
}

impl BdfElement for Bitmap {}

impl<'a> fmt::Display for ForBdf<'a, Bitmap> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BITMAP\n")?;

        for row in self.0.rows() {
            for byte in row.to_bytes() {
                write!(f, "{:02X}", byte)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

//

#[derive(Copy, Clone)]
pub struct XYPair {
    pub x: u32,
    pub y: u32,
}

impl BdfElement for XYPair {}

impl<'a> fmt::Display for ForBdf<'a, XYPair> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.0.x, self.0.y,)
    }
}

#[derive(Copy, Clone)]
pub enum WritingMetrics {
    Normal = 0,
    Alternate,
    Both,
}

impl BdfElement for WritingMetrics {}

impl<'a> fmt::Display for ForBdf<'a, WritingMetrics> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self.0 as u8)
    }
}

#[derive(Copy, Clone)]
pub struct BoundingBox {
    pub width: u32,
    pub height: u32,
    pub x_offset: i32,
    pub y_offset: i32,
}

impl BdfElement for BoundingBox {}

impl<'a> fmt::Display for ForBdf<'a, BoundingBox> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{} {} {} {}",
               self.0.width,
               self.0.height,
               self.0.x_offset,
               self.0.y_offset,)
    }
}

// #[derive(Debug)]
pub struct Glyph {
    pub name: String,
    pub codepoint: char,
    pub bounding_box: BoundingBox,
    pub bitmap: Bitmap,

    pub metrics: WritingMetrics,

    pub scalable_width: Option<XYPair>,
    pub device_width: Option<XYPair>,
    pub scalable_width_alt: Option<XYPair>,
    pub device_width_alt: Option<XYPair>,
    pub vector: Option<XYPair>,
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

impl BdfElement for Glyph {}

impl<'a> fmt::Display for ForBdf<'a, Glyph> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let glyph = &self.0;

        write!(f, "STARTCHAR {}\n", glyph.name)?;
        write!(f, "ENCODING {}\n", glyph.codepoint as u32)?;
        write!(f, "BBX {}\n", glyph.bounding_box.for_bdf())?;
        // TODO only do if metrics != normal ?
        write!(f, "METRICSSET {}\n", glyph.metrics.for_bdf())?;
        if let &Some(pair) = &glyph.scalable_width {
            write!(f, "SWIDTH {}\n", pair.for_bdf())?;
        }
        if let &Some(pair) = &glyph.device_width {
            write!(f, "DWIDTH {}\n", pair.for_bdf())?;
        }
        if let &Some(pair) = &glyph.scalable_width_alt {
            write!(f, "SWIDTH1 {}\n", pair.for_bdf())?;
        }
        if let &Some(pair) = &glyph.device_width_alt {
            write!(f, "DWIDTH1 {}\n", pair.for_bdf())?;
        }
        if let &Some(pair) = &glyph.vector {
            write!(f, "VVECTOR {}\n", pair.for_bdf())?;
        }
        write!(f, "{}", glyph.bitmap.for_bdf())?;
        write!(f, "ENDCHAR\n")
    }
}

#[derive(Copy, Clone)]
pub struct FontSize {
    pub point_size: u16,
    pub x_dpi: u16,
    pub y_dpi: u16,
}

impl BdfElement for FontSize {}

impl<'a> fmt::Display for ForBdf<'a, FontSize> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{} {} {}",
               self.0.point_size,
               self.0.x_dpi,
               self.0.y_dpi,)
    }
}

#[derive(Clone)]
pub enum PropertyValue {
    Str(String),
    Int(i32),
}

impl BdfElement for PropertyValue {}

impl<'a> fmt::Display for ForBdf<'a, PropertyValue> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            PropertyValue::Str(s) => write!(f, "{}", s.replace("\"", "\"\"")),
            PropertyValue::Int(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Clone)]
pub struct Property {
    pub name: String,
    pub value: PropertyValue,
}

impl BdfElement for Property {}

impl<'a> fmt::Display for ForBdf<'a, Property> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.0.name, self.0.value.for_bdf())
    }
}

// #[derive(Debug)]
pub struct Font {
    pub bdf_version: String,
    pub name: String,
    pub size: FontSize,
    pub bounding_box: BoundingBox,
    pub metrics: WritingMetrics,

    pub comments: Vec<String>,
    pub properties: Vec<Property>,
    pub glyphs: Vec<Glyph>,

    pub content_version: Option<String>,
    pub scalable_width: Option<XYPair>,
    pub device_width: Option<XYPair>,
    pub scalable_width_alt: Option<XYPair>,
    pub device_width_alt: Option<XYPair>,
    pub vector: Option<XYPair>,
}

impl Font {
    pub fn new(name: &str, size: FontSize, bounding_box: BoundingBox) -> Self {
        Self {
            bdf_version: String::from("2.2"),
            name: String::from(name),
            size,
            bounding_box,
            metrics: WritingMetrics::Normal,

            comments: Vec::new(),
            properties: Vec::new(),
            glyphs: Vec::new(),

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

impl BdfElement for Font {}

impl<'a> fmt::Display for ForBdf<'a, Font> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let font = &self.0;

        write!(f, "STARTFONT {}\n", font.bdf_version)?;
        write!(f, "FONT {}\n", font.name)?;
        for comment in &font.comments {
            write!(f, "COMMENT {}\n", comment)?;
        }
        if let Some(cv) = &font.content_version {
            write!(f, "CONTENTVERSION {}\n", cv)?;
        }
        write!(f, "SIZE {}\n", font.size.for_bdf())?;
        write!(f, "FONTBOUNDINGBOX {}\n", font.bounding_box.for_bdf())?;
        // TODO only do if metrics != normal ?
        write!(f, "METRICSSET {}\n", font.metrics.for_bdf())?;
        if let &Some(pair) = &font.scalable_width {
            write!(f, "SWIDTH {}\n", pair.for_bdf())?;
        }
        if let &Some(pair) = &font.device_width {
            write!(f, "DWIDTH {}\n", pair.for_bdf())?;
        }
        if let &Some(pair) = &font.scalable_width_alt {
            write!(f, "SWIDTH1 {}\n", pair.for_bdf())?;
        }
        if let &Some(pair) = &font.device_width_alt {
            write!(f, "DWIDTH1 {}\n", pair.for_bdf())?;
        }
        if let &Some(pair) = &font.vector {
            write!(f, "VVECTOR {}\n", pair.for_bdf())?;
        }
        if font.properties.len() > 0 {
            write!(f, "STARTPROPERTIES {}\n", font.properties.len())?;
            for property in &font.properties {
                write!(f, "{}", property.for_bdf())?;
            }
            write!(f, "ENDPROPERTIES\n")?;
        }
        if font.glyphs.len() > 0 {
            write!(f, "CHARS {}\n", font.glyphs.len())?;
            for glyph in &font.glyphs {
                write!(f, "{}", glyph.for_bdf())?;
            }
        }
        write!(f, "ENDFONT\n")
    }
}
