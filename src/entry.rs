use crate::{
    bdf::{
        self,
        BoundingBox,
        Property,
        WritingMetrics,
        FontSize,
    },
    bitmap::Bitmap,
};

// #[derive(PartialEq, Eq, Clone, Debug)]
pub enum Entry {
    StartFont(String),
    Comment(String),
    ContentVersion(String),
    Font(String),
    Size(FontSize),
    Chars(usize),
    FontBoundingBox(BoundingBox),
    EndFont,

    StartProperties(usize),
    Property(String, Property),
    EndProperties,

    StartChar(String),
    Encoding(char),
    MetricsSet(WritingMetrics),
    ScalableWidth(u32, u32),
    DeviceWidth(u32, u32),
    ScalableWidthAlt(u32, u32),
    DeviceWidthAlt(u32, u32),
    Vector(u32, u32),
    BoundingBox(BoundingBox),
    Bitmap(Bitmap),
    EndChar,
}

impl From<Entry> for String {
    fn from(entry: Entry) -> Self {
        use Entry::*;

        match entry {
            StartFont(s) => format!("STARTFONT {}", s),
            Comment(s) => format!("COMMENT {}", s),
            ContentVersion(s) => format!("CONTENTVERSION {}", s),
            Font(s) => format!("FONT {}", s),
            Size(FontSize { point_size, x_dpi, y_dpi }) => {
                format!("SIZE {} {} {}", point_size, x_dpi, y_dpi)
            },
            Chars(u) => format!("CHARS {}", u),
            FontBoundingBox(b) => {
                format!("FONTBOUNDINGBOX {} {} {} {}",
                    b.width,
                    b.height,
                    b.x_offset,
                    b.y_offset,)
            },
            EndFont => String::from("ENDFONT"),
            StartProperties(u) => format!("STARTPROPERTIES {}", u),
            Property(name, bdf::Property::Str(s)) => {
                format!("{} {}", name, s.replace("\"", "\"\""))
            },
            Property(name, bdf::Property::Int(i)) => {
                format!("{} {}", name, i)
            },
            EndProperties => String::from("ENDPROPERTIES"),
            StartChar(s) => format!("STARTCHAR {}", s),
            Encoding(c) => format!("ENCODING {}", c as u32),
            MetricsSet(m) => format!("METRICSSET {}", m as u8),
            ScalableWidth(x, y) => format!("SWIDTH {} {}", x, y),
            DeviceWidth(x, y) => format!("DWIDTH {} {}", x, y),
            ScalableWidthAlt(x, y) => format!("SWIDTH1 {} {}", x, y),
            DeviceWidthAlt(x, y) => format!("DWIDTH1 {} {}", x, y),
            Vector(x, y) => format!("VVECTOR {} {}", x, y),
            BoundingBox(b) => {
                format!("BBX {} {} {} {}",
                    b.width,
                    b.height,
                    b.x_offset,
                    b.y_offset,)
            },
            Bitmap(b) => {
                // TODO
                String::new()
            }
            EndChar => String::from("ENDCHAR"),
        }
    }
}
