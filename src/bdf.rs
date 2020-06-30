use std::fmt;
use std::str::{
    FromStr,
    SplitWhitespace
};

use bit_vec::BitVec;

//

pub struct ForBdf<'a, T>(&'a T);

pub trait BdfElement: Sized {
    fn for_bdf(&self) -> ForBdf<Self> {
        ForBdf(self)
    }
}

//

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

pub struct ParseError {
    pub desired: &'static str,
}

impl ParseError {
    pub fn new(desired: &'static str) -> Self {
        Self {
            desired,
        }
    }
}

fn parse_to_parts(s: &str, n: usize) -> Option<SplitWhitespace> {
    let mut parts = s.split_whitespace();
    let parts_ct = parts.clone().count();
    if parts_ct == n {
        Some(parts)
    } else {
        None
    }
}

//

// TODO handle float vs integer for scalable vs device width
#[derive(Copy, Clone)]
pub struct XYPair {
    pub x: u32,
    pub y: u32,
}

impl XYPair {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl FromStr for XYPair {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let desired = "X:integer Y:integer";
        let mut parts = match parse_to_parts(s, 2) {
            Some(p) => p,
            None    => return Err(ParseError::new(desired)),
        };

        let x = parts.next().unwrap();
        let y = parts.next().unwrap();

        let x = match x.parse() {
            Ok(v) => v,
            Err(e) => return Err(ParseError::new(desired)),
        };

        let y = match y.parse() {
            Ok(v) => v,
            Err(e) => return Err(ParseError::new(desired)),
        };

        Ok(Self::new(x, y))
    }
}

impl BdfElement for XYPair {}

impl<'a> fmt::Display for ForBdf<'a, XYPair> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.0.x, self.0.y,)
    }
}

//

#[derive(Copy, Clone)]
pub enum WritingMetrics {
    Normal = 0,
    Alternate,
    Both,
}

impl BdfElement for WritingMetrics {}

impl FromStr for WritingMetrics {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let desired = "N:integer(0, 1 or 2)";

        let wm = match s.parse() {
            Ok(0) => WritingMetrics::Normal,
            Ok(1) => WritingMetrics::Alternate,
            Ok(2) => WritingMetrics::Both,
            _ => return Err(ParseError::new(desired)),
        };

        Ok(wm)
    }
}

impl<'a> fmt::Display for ForBdf<'a, WritingMetrics> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self.0 as u8)
    }
}

//

#[derive(Copy, Clone)]
pub struct BoundingBox {
    pub width: u32,
    pub height: u32,
    pub x_offset: i32,
    pub y_offset: i32,
}

impl BoundingBox {
    pub fn new(width: u32, height: u32, x_offset: i32, y_offset: i32) -> Self {
        Self {
            width,
            height,
            x_offset,
            y_offset,
        }
    }
}

impl FromStr for BoundingBox {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let desired = "W:integer H:integer X:integer Y:integer";

        let mut parts = match parse_to_parts(s, 4) {
            Some(p) => p,
            None    => return Err(ParseError::new(desired)),
        };

        let w = parts.next().unwrap();
        let h = parts.next().unwrap();
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();

        let w = match w.parse() {
            Ok(v) => v,
            Err(e) => return Err(ParseError::new(desired)),
        };

        let h = match h.parse() {
            Ok(v) => v,
            Err(e) => return Err(ParseError::new(desired)),
        };

        let x = match x.parse() {
            Ok(v) => v,
            Err(e) => return Err(ParseError::new(desired)),
        };

        let y = match y.parse() {
            Ok(v) => v,
            Err(e) => return Err(ParseError::new(desired)),
        };

        Ok(Self::new(w, h, x, y))
    }
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

//

// #[derive(Debug)]
// TODO handle encodingwith two integers
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
    // TODO i think these should be floats
    pub point_size: u32,
    pub x_dpi: u32,
    pub y_dpi: u32,
}

impl FontSize {
    pub fn new(point_size: u32, x_dpi: u32, y_dpi: u32) -> Self {
        Self {
            point_size,
            x_dpi,
            y_dpi,
        }
    }
}

impl FromStr for FontSize {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let desired = "PT:number X:number Y:number";

        let mut parts = match parse_to_parts(s, 3) {
            Some(p) => p,
            None    => return Err(ParseError::new(desired)),
        };

        let p = parts.next().unwrap();
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();

        let p = match p.parse() {
            Ok(v) => v,
            Err(e) => return Err(ParseError::new(desired)),
        };

        let x = match x.parse() {
            Ok(v) => v,
            Err(e) => return Err(ParseError::new(desired)),
        };

        let y = match y.parse() {
            Ok(v) => v,
            Err(e) => return Err(ParseError::new(desired)),
        };

        Ok(Self::new(p, x, y))
    }
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

impl FromStr for PropertyValue {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let desired = "\"string\"|i:integer";
        if s.starts_with('"') {
            let string = match (&s[1..]).rfind('"') {
                Some(n) => s[1..n].replace("\"\"", "\""),
                None    => return Err(ParseError::new(desired)),
            };
            Ok(PropertyValue::Str(string))
        } else {
            let i = match s.parse() {
                Ok(v) => v,
                Err(e) => return Err(ParseError::new(desired)),
            };
            Ok(PropertyValue::Int(i))
        }
    }
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

impl Property {
    pub fn new(name: &str, value: &PropertyValue) -> Self {
        Self {
            name: String::from(name),
            value: value.clone(),
        }
    }
}

impl BdfElement for Property {}

impl<'a> fmt::Display for ForBdf<'a, Property> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}\n", self.0.name, self.0.value.for_bdf())
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

    pub content_version: Option<i32>,
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

//

pub enum Error {
    MissingValue(usize, String),
    UnexpectedEntry(usize, String),
    // TODO incl parse error
    ParseError(usize),
    NotFound(&'static str),
}

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    pub fn parse(input: &str) -> Result<Font, Error> {
        use Error::*;

        #[derive(Copy, Clone)]
        enum ParseState {
            Empty,
            InFont,
            InChar,
            // InBitmap, ?
            InProperties,
        }

        let mut ret = Font::new("",
            FontSize::new(0, 0, 0),
            BoundingBox::new(0, 0, 0, 0));

        let mut found_bdf_version = false;
        let mut found_name = false;
        let mut found_size = false;
        let mut found_bbox = false;

        let mut state = ParseState::Empty;

        let mut properties_ct: Option<usize> = None;
        let mut glyphs_ct: Option<usize> = None;
        let mut bitmap_len: Option<usize> = None;

        let mut curr_glyph: Option<Glyph> = None;
        let mut found_glyph_name = false;
        let mut found_glyph_char = false;
        let mut found_glyph_bbox = false;
        let mut found_glyph_bmap = false;
        let mut curr_bitmap: Option<Glyph> = None;

        let mut main_bbox: Option<BoundingBox> = None;
        let mut curr_bbox: Option<BoundingBox> = None;

        let lines = input.trim().split('\n');
        let lines_ct = lines.clone().count();
        for (line_num, long_line) in lines.enumerate() {
            let line = long_line.trim();
            let (id, rest) = match line.find(char::is_whitespace) {
                Some(n) => (&line[0..n], Some((&line[n..]).trim())),
                None    => (line, None),
            };

            match (state, id, rest) {
                (_, "COMMENT", Some(s)) => ret.comments.push(String::from(s)),

                // TODO check char ct
                //      verify font
                (ParseState::InFont, "ENDFONT", _)             => state = ParseState::Empty,
                (_, id @ "ENDFONT", _)                         => return Err(UnexpectedEntry(line_num, String::from(id))),
                // TODO check properties len
                (ParseState::InProperties, "ENDPROPERTIES", _) => state = ParseState::InFont,
                (_, id @ "ENDPROPERITES", _)                   => return Err(UnexpectedEntry(line_num, String::from(id))),
                // TODO verify glyph
                (ParseState::InChar, "ENDCHAR", _)             => state = ParseState::InFont,
                (_, id @ "ENDCHAR", _)                         => return Err(UnexpectedEntry(line_num, String::from(id))),
                (_, id, None) => return Err(MissingValue(line_num, String::from(id))),

                (ParseState::Empty, "STARTFONT", Some(v)) => {
                    ret.bdf_version = String::from(v);
                    found_bdf_version = true;
                    state = ParseState::InFont;
                },
                (ParseState::Empty, id, _) => return Err(UnexpectedEntry(line_num, String::from(id))),

                (ParseState::InFont, "FONT", Some(n)) => {
                    ret.name = String::from(n);
                    found_name = true;
                },
                (ParseState::InFont, "CONTENTVERSION", Some(s)) => {
                    match s.parse() {
                        Ok(i) => ret.content_version = Some(i),
                        Err(_) => return Err(ParseError(line_num)),
                    }
                },
                (ParseState::InFont, "SIZE", Some(s)) => {
                    match s.parse() {
                        Ok(s) => {
                            ret.size = s;
                            found_size = true;
                        }
                        Err(_) => return Err(ParseError(line_num)),
                    }
                },
                (ParseState::InFont, "FONTBOUNDINGBOX", Some(s)) => {
                    match s.parse() {
                        Ok(s) => {
                            ret.bounding_box = s;
                            found_bbox = true;
                            main_bbox = Some(s);
                        }
                        Err(_) => return Err(ParseError(line_num)),
                    }
                },
                (ParseState::InFont, "CHARS", Some(s)) => {
                    // TODO: note, doesnt account for wether this is followed by the charset or not
                    match s.parse() {
                        Ok(s) => glyphs_ct = Some(s),
                        Err(_) => return Err(ParseError(line_num)),
                    }
                },
                (ParseState::InFont, "STARTCHAR", Some(s)) => {
                    curr_glyph = Some(Glyph::new(s, 0 as char, BoundingBox::new(0, 0, 0, 0), Bitmap::new(0, 0)));
                    found_glyph_name = true;
                    found_glyph_char = false;
                    found_glyph_bbox = false;
                    found_glyph_bmap = false;
                    state = ParseState::InChar;
                }
                (ParseState::InFont, id, _) => return Err(UnexpectedEntry(line_num, String::from(id))),

                (ParseState::InChar, "ENCODING", Some(s)) => {
                }
                _ => {}

            }
        }

        if !found_bdf_version {
            Err(NotFound("bdf version"))
        } else if !found_name {
            Err(NotFound("name"))
        } else if !found_size {
            Err(NotFound("size"))
        } else if !found_bbox {
            Err(NotFound("font bounding box"))
        } else {
            Ok(ret)
        }
    }
}
