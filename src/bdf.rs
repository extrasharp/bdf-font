use std::{
    ops::{
        Deref,
        DerefMut,
    },
    convert::TryFrom,
    fmt,
    str::{
        FromStr,
        SplitWhitespace
    },
};

use bit_vec::BitVec;

//

pub struct ForBdf<'a, T: ?Sized>(&'a T);

pub trait BdfValue {
    fn desired() -> &'static str;

    fn for_bdf(&self) -> ForBdf<Self> {
        ForBdf(self)
    }
}

pub trait BdfBlock {
    fn for_bdf(&self) -> ForBdf<Self> {
        ForBdf(self)
    }
}

//

fn split_to_parts(s: &str, n: usize) -> Result<SplitWhitespace, usize> {
    let parts = s.split_whitespace();
    let parts_ct = parts.clone().count();
    if parts_ct == n {
        Ok(parts)
    } else {
        Err(parts_ct)
    }
}

//

#[derive(Copy, Clone, Debug)]
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

impl BdfValue for XYPair {
    fn desired() -> &'static str {
        "X:integer Y:integer"
    }
}

impl FromStr for XYPair {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = split_to_parts(s, 2).or(Err(Self::desired()))?;

        let x = parts.next().unwrap();
        let y = parts.next().unwrap();

        let x = x.parse().or(Err(Self::desired()))?;
        let y = y.parse().or(Err(Self::desired()))?;

        Ok(Self::new(x, y))
    }
}

impl<'a> fmt::Display for ForBdf<'a, XYPair> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.0.x, self.0.y,)
    }
}

//

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum MetricsSet {
    Normal = 0,
    Alternate,
    Both,
}

impl BdfValue for MetricsSet {
    fn desired() -> &'static str {
        "N:integer(0, 1 or 2)"
    }
}

impl FromStr for MetricsSet {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let wm = match s.parse() {
            Ok(0) => MetricsSet::Normal,
            Ok(1) => MetricsSet::Alternate,
            Ok(2) => MetricsSet::Both,
            _ => return Err(Self::desired()),
        };

        Ok(wm)
    }
}

impl<'a> fmt::Display for ForBdf<'a, MetricsSet> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self.0 as u8)
    }
}

//

#[derive(Copy, Clone, Debug)]
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

impl BdfValue for BoundingBox {
    fn desired() -> &'static str {
        "W:integer H:integer X:integer Y:integer"
    }
}

impl FromStr for BoundingBox {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = split_to_parts(s, 4).or(Err(Self::desired()))?;

        let w = parts.next().unwrap();
        let h = parts.next().unwrap();
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();

        let w = w.parse().or(Err(Self::desired()))?;
        let h = h.parse().or(Err(Self::desired()))?;
        let x = x.parse().or(Err(Self::desired()))?;
        let y = y.parse().or(Err(Self::desired()))?;

        Ok(Self::new(w, h, x, y))
    }
}

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

#[derive(Copy, Clone, Debug)]
pub struct FontSize {
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

impl BdfValue for FontSize {
    fn desired() -> &'static str {
        "PT:number X:number Y:number"
    }
}

impl FromStr for FontSize {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = split_to_parts(s, 3).or(Err(Self::desired()))?;

        let p = parts.next().unwrap();
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();

        let p = p.parse().or(Err(Self::desired()))?;
        let x = x.parse().or(Err(Self::desired()))?;
        let y = y.parse().or(Err(Self::desired()))?;

        Ok(Self::new(p, x, y))
    }
}

impl<'a> fmt::Display for ForBdf<'a, FontSize> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{} {} {}",
               self.0.point_size,
               self.0.x_dpi,
               self.0.y_dpi,)
    }
}

//

#[derive(Clone, Debug)]
pub enum PropertyValue {
    Str(String),
    Int(i32),
}

impl BdfValue for PropertyValue {
    fn desired() -> &'static str {
        "s:\"string\"|i:integer"
    }
}

impl FromStr for PropertyValue {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('"') {
            let string = match (&s[1..]).rfind('"') {
                Some(n) => s[1..n].replace("\"\"", "\""),
                None    => return Err(Self::desired()),
            };
            Ok(PropertyValue::Str(string))
        } else {
            let i = s.parse().or(Err(Self::desired()))?;
            Ok(PropertyValue::Int(i))
        }
    }
}

impl<'a> fmt::Display for ForBdf<'a, PropertyValue> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            PropertyValue::Str(s) => write!(f, "\"{}\"", s.replace("\"", "\"\"")),
            PropertyValue::Int(i) => write!(f, "{}", i),
        }
    }
}

//

#[derive(Clone, Debug)]
pub struct BitmapRow(BitVec);

impl Deref for BitmapRow {
    type Target = BitVec;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BitmapRow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl BdfValue for BitmapRow {
    fn desired() -> &'static str {
        "bytestring"
    }
}

impl FromStr for BitmapRow {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 != 0 {
            return Err(Self::desired());
        }

        let mut buf: Vec<u8> = Vec::new();
        buf.reserve(s.len() / 2);

        for i in 0..(s.len() / 2) {
            buf.push(u8::from_str_radix(&s[i..=(i+1)], 16).or_else(|_| Err(Self::desired()))?);
        }

        Ok(Self(BitVec::from_bytes(&buf)))
    }
}

impl<'a> fmt::Display for ForBdf<'a, BitmapRow> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0.to_bytes() {
            write!(f, "{:02X}", byte)?;
        }

        Ok(())
    }
}

//

#[derive(Clone, Debug)]
pub struct Bitmap {
    width: usize,
    height: usize,
    data: Vec<BitmapRow>
}

impl Bitmap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![BitmapRow(BitVec::from_elem(width, false)); height]
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn rows(&self) -> &[BitmapRow] {
        &self.data
    }

    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.data[y][x])
        }
    }

    pub fn set(&mut self, x: usize, y: usize, to: bool) {
        if x >= self.width || y >= self.height {
            return;
        } else {
            self.data[y].set(x, to);
        }
    }
}

impl BdfBlock for Bitmap {}

impl<'a> fmt::Display for ForBdf<'a, Bitmap> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", ids::BITMAP)?;

        for row in self.0.rows() {
            write!(f, "{}\n", row.for_bdf())?;
        }

        Ok(())
    }
}

//

#[derive(Debug)]
pub struct Glyph {
    pub name: String,
    pub codepoint: char,
    pub bounding_box: BoundingBox,
    pub bitmap: Bitmap,

    pub metrics: MetricsSet,

    pub scalable_width: Option<XYPair>,
    pub device_width: Option<XYPair>,
    pub scalable_width_alt: Option<XYPair>,
    pub device_width_alt: Option<XYPair>,
    pub vector: Option<XYPair>,
}

impl BdfBlock for Glyph {}

impl<'a> fmt::Display for ForBdf<'a, Glyph> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let glyph = &self.0;

        write!(f, "{} {}\n", ids::STARTCHAR, glyph.name)?;
        write!(f, "{} {}\n", ids::ENCODING, glyph.codepoint as u32)?;
        write!(f, "{} {}\n", ids::BBX, glyph.bounding_box.for_bdf())?;
        if glyph.metrics != MetricsSet::Normal {
            write!(f, "{} {}\n", ids::METRICSSET, glyph.metrics.for_bdf())?;
        }
        if let &Some(pair) = &glyph.scalable_width {
            write!(f, "{} {}\n", ids::SWIDTH, pair.for_bdf())?;
        }
        if let &Some(pair) = &glyph.device_width {
            write!(f, "{} {}\n", ids::DWIDTH, pair.for_bdf())?;
        }
        if let &Some(pair) = &glyph.scalable_width_alt {
            write!(f, "{} {}\n", ids::SWIDTH1, pair.for_bdf())?;
        }
        if let &Some(pair) = &glyph.device_width_alt {
            write!(f, "{} {}\n", ids::DWIDTH1, pair.for_bdf())?;
        }
        if let &Some(pair) = &glyph.vector {
            write!(f, "{} {}\n", ids::VVECTOR, pair.for_bdf())?;
        }
        write!(f, "{}", glyph.bitmap.for_bdf())?;
        write!(f, "{}\n", ids::ENDCHAR)
    }
}

//

#[derive(Clone, Debug)]
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

impl BdfBlock for Property {}

impl<'a> fmt::Display for ForBdf<'a, Property> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}\n", self.0.name, self.0.value.for_bdf())
    }
}

//

#[derive(Debug)]
pub struct Font {
    pub bdf_version: String,
    pub name: String,
    pub size: FontSize,
    pub bounding_box: BoundingBox,
    pub metrics: MetricsSet,

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

impl BdfBlock for Font {}

impl<'a> fmt::Display for ForBdf<'a, Font> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let font = &self.0;

        write!(f, "{} {}\n", ids::STARTFONT, font.bdf_version)?;
        write!(f, "{} {}\n", ids::FONT, font.name)?;
        for comment in &font.comments {
            write!(f, "{} {}\n", ids::COMMENT, comment)?;
        }
        if let Some(cv) = &font.content_version {
            write!(f, "{} {}\n", ids::CONTENTVERSION, cv)?;
        }
        write!(f, "{} {}\n", ids::SIZE, font.size.for_bdf())?;
        write!(f, "{} {}\n", ids::FONTBOUNDINGBOX, font.bounding_box.for_bdf())?;
        if font.metrics != MetricsSet::Normal {
            write!(f, "{} {}\n", ids::METRICSSET, font.metrics.for_bdf())?;
        }
        if let &Some(pair) = &font.scalable_width {
            write!(f, "{} {}\n", ids::SWIDTH, pair.for_bdf())?;
        }
        if let &Some(pair) = &font.device_width {
            write!(f, "{} {}\n", ids::DWIDTH, pair.for_bdf())?;
        }
        if let &Some(pair) = &font.scalable_width_alt {
            write!(f, "{} {}\n", ids::SWIDTH1, pair.for_bdf())?;
        }
        if let &Some(pair) = &font.device_width_alt {
            write!(f, "{} {}\n", ids::DWIDTH1, pair.for_bdf())?;
        }
        if let &Some(pair) = &font.vector {
            write!(f, "{} {}\n", ids::VVECTOR, pair.for_bdf())?;
        }
        if font.properties.len() > 0 {
            write!(f, "{} {}\n", ids::STARTPROPERTIES, font.properties.len())?;
            for property in &font.properties {
                write!(f, "{}", property.for_bdf())?;
            }
            write!(f, "{}\n", ids::ENDPROPERTIES)?;
        }
        if font.glyphs.len() > 0 {
            write!(f, "{} {}\n", ids::CHARS, font.glyphs.len())?;
            for glyph in &font.glyphs {
                write!(f, "{}", glyph.for_bdf())?;
            }
        }
        write!(f, "{}\n", ids::ENDFONT)
    }
}

//

#[derive(Clone, Debug)]
struct BitmapShell {
    pub width: usize,
    pub height: usize,
    pub data: Vec<BitmapRow>
}

impl BitmapShell {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            data: Vec::new(),
        }
    }

    fn verify(&self) -> Result<(), Error> {
        Ok(())
    }

    fn to_bitmap(self) -> Result<Bitmap, Error> {
        self.verify()?;

        Ok(Bitmap {
            width: self.width,
            height: self.height,
            data: self.data,
        })
    }
}

#[derive(Debug)]
struct GlyphShell<'a> {
    pub name: Option<&'a str>,
    pub codepoint: Option<char>,
    pub bounding_box: Option<BoundingBox>,
    pub bitmap: BitmapShell,

    pub metrics: Option<MetricsSet>,

    pub scalable_width: Option<XYPair>,
    pub device_width: Option<XYPair>,
    pub scalable_width_alt: Option<XYPair>,
    pub device_width_alt: Option<XYPair>,
    pub vector: Option<XYPair>,
}

impl<'a> GlyphShell<'a> {
    fn new() -> Self {
        Self {
            name: None,
            codepoint: None,
            bounding_box: None,
            bitmap: BitmapShell::new(),
            metrics: None,
            scalable_width: None,
            device_width: None,
            scalable_width_alt: None,
            device_width_alt: None,
            vector: None,
        }
    }

    fn verify(&self) -> Result<(), Error> {
        use Error::*;

        if self.codepoint.is_none() {
            return Err(GlyphValidation('\0', "codepoint not found"));
        }

        let codepoint = self.codepoint.unwrap();

        if self.name.is_none() {
            return Err(GlyphValidation(codepoint, "name not found"));
        } else if self.bounding_box.is_none() {
            return Err(GlyphValidation(codepoint, "bounding box not found"));
        }

        match self.metrics {
            None | Some(MetricsSet::Normal) => {
                if !(self.scalable_width_alt.is_none() &&
                     self.device_width_alt.is_none()) {
                    return Err(GlyphValidation(codepoint, "glyph with normal metrics cannot have alternate widths"));
                }
            }
            Some(_) => {
                if !(self.scalable_width_alt.is_some() &&
                     self.device_width_alt.is_some()) {
                    return Err(GlyphValidation(codepoint, "glyph with alternate metrics must have alternate widths"));
                }
            }
        }

        self.bitmap.verify()?;

        Ok(())
    }

    fn to_glyph(self) -> Result<Glyph, Error> {
        self.verify()?;

        let codepoint = self.codepoint.unwrap();
        let bitmap = self.bitmap.to_bitmap()?;

        Ok(Glyph {
            name: String::from(self.name.unwrap()),
            codepoint,
            bounding_box: self.bounding_box.unwrap(),
            bitmap,
            metrics: self.metrics.unwrap_or(MetricsSet::Normal),

            scalable_width: self.scalable_width,
            device_width: self.device_width,
            scalable_width_alt: self.scalable_width_alt,
            device_width_alt: self.device_width_alt,
            vector: self.vector,
        })
    }
}

#[derive(Debug)]
enum PropertyValueShell<'a> {
    Str(&'a str),
    Int(i32),
}

impl<'a> PropertyValueShell<'a> {
    fn verify(&self) -> Result<(), Error> {
        Ok(())
    }

    fn to_property_value(self) -> Result<PropertyValue, Error> {
        self.verify()?;
        Ok(match self {
            Self::Str(s) => PropertyValue::Str(s.replace("\"\"", "\"")),
            Self::Int(i) => PropertyValue::Int(i),
        })
    }
}

#[derive(Debug)]
struct PropertyShell<'a> {
    pub name: &'a str,
    pub value: PropertyValueShell<'a>
}

impl<'a> PropertyShell<'a> {
    fn new(name: &'a str, value: PropertyValueShell<'a>) -> Self {
        Self {
            name,
            value,
        }
    }

    fn verify(&self) -> Result<(), Error> {
        self.value.verify()
    }

    fn to_property(self) -> Result<Property, Error> {
        self.verify()?;
        Ok(Property {
            name: String::from(self.name),
            value: self.value.to_property_value()?,
        })
    }
}

#[derive(Debug)]
struct FontShell<'a> {
    pub bdf_version: Option<&'a str>,
    pub name: Option<&'a str>,
    pub size: Option<FontSize>,
    pub bounding_box: Option<BoundingBox>,
    pub metrics: Option<MetricsSet>,

    pub comments: Vec<&'a str>,
    pub properties: Vec<PropertyShell<'a>>,
    pub glyphs: Vec<GlyphShell<'a>>,

    pub content_version: Option<i32>,
    pub scalable_width: Option<XYPair>,
    pub device_width: Option<XYPair>,
    pub scalable_width_alt: Option<XYPair>,
    pub device_width_alt: Option<XYPair>,
    pub vector: Option<XYPair>,
}

impl<'a> FontShell<'a> {
    fn new() -> Self {
        Self {
            bdf_version: None,
            name: None,
            size: None,
            bounding_box: None,
            metrics: None,
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

    fn verify(&self) -> Result<(), Error> {
        use Error::*;

        if self.bdf_version.is_none() {
            return Err(FontValidation("bdf version not found"));
        } else if self.name.is_none() {
            return Err(FontValidation("name not found"));
        } else if self.size.is_none() {
            return Err(FontValidation("size not found"));
        } else if self.bounding_box.is_none() {
            return Err(FontValidation("bounding box not found"));
        }

        match self.metrics {
            None | Some(MetricsSet::Normal) => {
                if !(self.scalable_width_alt.is_none() &&
                     self.device_width_alt.is_none()) {
                    return Err(FontValidation("font with normal metrics cannot have alternate widths"));
                }
            }
            Some(_) => {
                if !(self.scalable_width_alt.is_some() &&
                     self.device_width_alt.is_some()) {
                    return Err(FontValidation("font with alternate metrics must have alternate widths"));
                }
            }
        }

        let _ = self.glyphs.iter()
                           .map(GlyphShell::verify)
                           .collect::<Result<Vec<_>, _>>()?;
        let _ = self.properties.iter()
                               .map(PropertyShell::verify)
                               .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    }

    fn to_font(self) -> Result<Font, Error> {
        self.verify()?;

        let glyphs = self.glyphs.into_iter()
                                .map(GlyphShell::to_glyph)
                                .collect::<Result<Vec<_>, _>>()?;
        let properties = self.properties.into_iter()
                                        .map(PropertyShell::to_property)
                                        .collect::<Result<Vec<_>, _>>()?;
        let comments = self.comments.into_iter()
                                    .map(String::from)
                                    .collect();

        Ok(Font {
            bdf_version: String::from(self.bdf_version.unwrap()),
            name: String::from(self.name.unwrap()),
            size: self.size.unwrap(),
            bounding_box: self.bounding_box.unwrap(),
            metrics: self.metrics.unwrap_or(MetricsSet::Normal),

            comments,
            properties,
            glyphs,

            content_version: self.content_version,
            scalable_width: self.scalable_width,
            device_width: self.device_width,
            scalable_width_alt: self.scalable_width_alt,
            device_width_alt: self.device_width_alt,
            vector: self.vector,
        })
    }
}

//

mod ids {
    pub const COMMENT: &str = "COMMENT";

    pub const STARTFONT: &str = "STARTFONT";
    pub const CONTENTVERSION: &str = "CONTENTVERSION";
    pub const FONT: &str = "FONT";
    pub const FONTBOUNDINGBOX: &str = "FONTBOUNDINGBOX";
    pub const METRICSSET: &str = "METRICSSET";
    pub const SIZE: &str = "SIZE";
    pub const SWIDTH: &str = "SWIDTH";
    pub const DWIDTH: &str = "DWIDTH";
    pub const SWIDTH1: &str = "SWIDTH1";
    pub const DWIDTH1: &str = "DWIDTH1";
    pub const VVECTOR: &str = "VVECTOR";
    pub const CHARS: &str = "CHARS";
    pub const ENDFONT: &str = "ENDFONT";

    pub const STARTPROPERTIES: &str = "STARTPROPERTIES";
    pub const ENDPROPERTIES: &str = "ENDPROPERTIES";

    pub const STARTCHAR: &str = "STARTCHAR";
    pub const ENCODING: &str = "ENCODING";
    pub const BBX: &str = "BBX";
    pub const BITMAP: &str = "BITMAP";
    pub const ENDCHAR: &str = "ENDCHAR";
}

#[derive(Debug)]
pub enum Error {
    MissingValue(String),
    UnexpectedEntry(String),
    MissingBoundingBox,
    InvalidCodepoint(u32),
    ParseError(&'static str),
    SpecialEncoding,

    FontValidation(&'static str),
    GlyphValidation(char, &'static str),
}

pub fn parse_font(input: &str) -> Result<Font, (usize, Error)> {
    use Error::*;

    #[derive(Eq, PartialEq, Copy, Clone)]
    enum ParseState {
        Empty,
        InFont,
        InProperties,
        InChars,
        InChar,
        InBitmap,
    }

    let mut state = ParseState::Empty;

    let mut bitmap_len: u32 = 0;

    let mut main_bbox: Option<BoundingBox> = None;
    let mut curr_bbox: Option<BoundingBox> = None;

    let mut f_shell = FontShell::new();

    let lines = input.trim().split('\n');
    let lines_ct = lines.clone().count();
    for (line_num, long_line) in lines.enumerate() {
        if long_line.chars().all(char::is_whitespace) {
            continue;
        }

        let line_num = line_num + 1;
        let line = long_line.trim();

        let (id, rest) = match line.find(char::is_whitespace) {
            Some(n) => (&line[0..n], Some((&line[n..]).trim())),
            None    => (line, None),
        };

        if state == ParseState::InBitmap {
            if bitmap_len == 0 {
                state = ParseState::InChar;
            } else {
                match (id, rest) {
                    (val, None) => {
                        let row = val.parse().or_else(|e| Err((line_num, ParseError(e))))?;

                        let g_shell = f_shell.glyphs.last_mut().unwrap();
                        g_shell.bitmap.data.push(row);

                        bitmap_len -= 1;
                        continue;
                    }
                    (_, Some(_)) => {
                        return Err((line_num, ParseError(BitmapRow::desired())));
                    }
                }
            }
        }

        match (state, id, rest) {
            (_, ids::COMMENT, Some(s)) => {
                f_shell.comments.push(s);
                continue;
            },

            (ParseState::InChars, id @ ids::ENDFONT, _) => {
                if line_num != lines_ct {
                    return Err((line_num, UnexpectedEntry(String::from(id))));
                }
                break;
            },
            (_, id @ ids::ENDFONT, _) => return Err((line_num, UnexpectedEntry(String::from(id)))),

            (ParseState::InProperties, ids::ENDPROPERTIES, _) => {
                state = ParseState::InFont;
                continue;
            },
            (_, id @ ids::ENDPROPERTIES, _) => return Err((line_num, UnexpectedEntry(String::from(id)))),

            (ParseState::InChar, ids::ENDCHAR, _) => {
                let g_shell = f_shell.glyphs.last_mut().unwrap();
                match g_shell.verify() {
                    Ok(()) => {},
                    Err(err) => return Err((line_num, err)),
                }
                state = ParseState::InChars;
                continue;
            },
            (ParseState::InChar, ids::BITMAP, _) => {
                let bbox = match (&main_bbox, &curr_bbox) {
                    (_, Some(bbox)) | (Some(bbox), None) => bbox,
                    (None, None) => return Err((line_num, MissingBoundingBox)),
                };

                let g_shell = f_shell.glyphs.last_mut().unwrap();
                g_shell.bitmap.width = bbox.width as usize;
                g_shell.bitmap.height = bbox.height as usize;
                g_shell.bitmap.data.reserve(bbox.height as usize);

                bitmap_len = bbox.height;
                state = ParseState::InBitmap;
                continue;
            },
            (_, id @ ids::ENDCHAR, _) => return Err((line_num, UnexpectedEntry(String::from(id)))),

            (_, id, None) => return Err((line_num, MissingValue(String::from(id)))),
            (_, _, Some(_)) => {}
        }

        let rest = rest.unwrap();

        match state {
            ParseState::Empty => match id {
                ids::STARTFONT => {
                    f_shell.bdf_version = Some(rest);
                    state = ParseState::InFont;
                },
                id => return Err((line_num, UnexpectedEntry(String::from(id)))),
            }
            ParseState::InFont => match id {
                ids::FONT => {
                    f_shell.name = Some(rest);
                },
                ids::CONTENTVERSION => {
                    let val = rest.parse().or_else(|_| Err((line_num, ParseError("integer"))))?;
                    f_shell.content_version = Some(val);
                },
                ids::SIZE => {
                    let val = rest.parse().or_else(|e| Err((line_num, ParseError(e))))?;
                    f_shell.size = Some(val);
                },
                ids::FONTBOUNDINGBOX => {
                    let val = rest.parse().or_else(|e| Err((line_num, ParseError(e))))?;
                    f_shell.bounding_box = Some(val);
                    main_bbox = Some(val);
                },
                ids::METRICSSET => {
                    let val = rest.parse().or_else(|e| Err((line_num, ParseError(e))))?;
                    f_shell.metrics = Some(val);
                },
                ids::SWIDTH => {
                    let val = rest.parse().or_else(|e| Err((line_num, ParseError(e))))?;
                    f_shell.scalable_width = Some(val);
                },
                ids::DWIDTH => {
                    let val = rest.parse().or_else(|e| Err((line_num, ParseError(e))))?;
                    f_shell.device_width = Some(val);
                },
                ids::SWIDTH1 => {
                    let val = rest.parse().or_else(|e| Err((line_num, ParseError(e))))?;
                    f_shell.scalable_width_alt = Some(val);
                },
                ids::DWIDTH1 => {
                    let val = rest.parse().or_else(|e| Err((line_num, ParseError(e))))?;
                    f_shell.device_width_alt = Some(val);
                },
                ids::VVECTOR => {
                    let val = rest.parse().or_else(|e| Err((line_num, ParseError(e))))?;
                    f_shell.vector = Some(val);
                },
                ids::STARTPROPERTIES => {
                    let val = rest.parse().or_else(|_| Err((line_num, ParseError("integer"))))?;
                    f_shell.properties.reserve(val);
                    state = ParseState::InProperties;
                }
                ids::CHARS => {
                    let val = rest.parse().or_else(|_| Err((line_num, ParseError("integer"))))?;
                    f_shell.glyphs.reserve(val);
                    state = ParseState::InChars;
                },
                id => return Err((line_num, UnexpectedEntry(String::from(id)))),
            }
            ParseState::InProperties => {
                use PropertyValueShell::*;

                if rest.starts_with('"') {
                    if rest.ends_with('"') {
                        f_shell.properties.push(PropertyShell::new(id, Str(&rest[1..(rest.len()-1)])));
                    } else {
                        return Err((line_num, ParseError(PropertyValue::desired())));
                    }
                } else {
                    match rest.parse() {
                        Ok(i) => f_shell.properties.push(PropertyShell::new(id, Int(i))),
                        Err(_) => return Err((line_num, ParseError(PropertyValue::desired()))),
                    };
                }
            }
            ParseState::InChars => match id {
                ids::STARTCHAR => {
                    f_shell.glyphs.push(GlyphShell::new());
                    let g_shell = f_shell.glyphs.last_mut().unwrap();
                    g_shell.name = Some(rest);
                    state = ParseState::InChar;
                }
                id => return Err((line_num, UnexpectedEntry(String::from(id)))),
            },
            ParseState::InChar => {
                let g_shell = f_shell.glyphs.last_mut().unwrap();

                match id {
                    ids::ENCODING => {
                        match rest.find(char::is_whitespace) {
                            Some(n) => match &rest[0..n].parse::<i64>() {
                                Ok(-1) => return Err((line_num, SpecialEncoding)),
                                _ => return Err((line_num, ParseError("-1 integer"))),
                            },
                            None => {}
                        }

                        match rest.parse::<u32>() {
                            Ok(u) => {
                                match char::try_from(u) {
                                    Ok(c) => g_shell.codepoint = Some(c),
                                    Err(_) => return Err((line_num, InvalidCodepoint(u))),
                                }
                            }
                            Err(_) => return Err((line_num, ParseError("integer"))),
                        };
                    },
                    ids::METRICSSET => {
                        let val = rest.parse().or_else(|e| Err((line_num, ParseError(e))))?;
                        g_shell.metrics = Some(val);
                    },
                    ids::SWIDTH => {
                        let val = rest.parse().or_else(|e| Err((line_num, ParseError(e))))?;
                        g_shell.scalable_width = Some(val);
                    },
                    ids::DWIDTH => {
                        let val = rest.parse().or_else(|e| Err((line_num, ParseError(e))))?;
                        g_shell.device_width = Some(val);
                    },
                    ids::SWIDTH1 => {
                        let val = rest.parse().or_else(|e| Err((line_num, ParseError(e))))?;
                        g_shell.scalable_width_alt = Some(val);
                    },
                    ids::DWIDTH1 => {
                        let val = rest.parse().or_else(|e| Err((line_num, ParseError(e))))?;
                        g_shell.device_width_alt = Some(val);
                    },
                    ids::VVECTOR => {
                        let val = rest.parse().or_else(|e| Err((line_num, ParseError(e))))?;
                        g_shell.vector = Some(val);
                    },
                    ids::BBX => {
                        let val = rest.parse().or_else(|e| Err((line_num, ParseError(e))))?;
                        g_shell.bounding_box = Some(val);
                        curr_bbox = Some(val);
                    },
                    id => return Err((line_num, UnexpectedEntry(String::from(id)))),
                }
            }
            ParseState::InBitmap => {
                // rest should be None
                unreachable!();
            }
        }
    }

    f_shell.to_font().or_else(|e| Err((lines_ct, e)))
}
