use std::fmt::{
    self,
    Write,
};

//

use crate::{
    bdf::{
        self,
        BoundingBox,
        Property,
        WritingMetrics,
        FontSize,
        Bitmap,
    },
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

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Entry::*;

        match self {
            StartFont(s)           => write!(f, "STARTFONT {}\n", s),
            Comment(s)             => write!(f, "COMMENT {}\n", s),
            ContentVersion(s)      => write!(f, "CONTENTVERSION {}\n", s),
            Font(s)                => write!(f, "FONT {}\n", s),
            Chars(u)               => write!(f, "CHARS {}\n", u),
            EndFont                => write!(f, "ENDFONT\n"),
            StartProperties(u)     => write!(f, "STARTPROPERTIES {}\n", u),
            EndProperties          => write!(f, "ENDPROPERTIES\n"),
            StartChar(s)           => write!(f, "STARTCHAR {}\n", s),
            &Encoding(c)           => write!(f, "ENCODING {}\n", c as u32),
            &MetricsSet(m)         => write!(f, "METRICSSET {}\n", m as u8),
            ScalableWidth(x, y)    => write!(f, "SWIDTH {} {}\n", x, y),
            DeviceWidth(x, y)      => write!(f, "DWIDTH {} {}\n", x, y),
            ScalableWidthAlt(x, y) => write!(f, "SWIDTH1 {} {}\n", x, y),
            DeviceWidthAlt(x, y)   => write!(f, "DWIDTH1 {} {}\n", x, y),
            Vector(x, y)           => write!(f, "VVECTOR {} {}\n", x, y),
            EndChar                => write!(f, "ENDCHAR\n"),
            Size(FontSize { point_size, x_dpi, y_dpi }) => {
                write!(f, "SIZE {} {} {}\n", point_size, x_dpi, y_dpi)
            },
            FontBoundingBox(b) => {
                write!(f, "FONTBOUNDINGBOX {} {} {} {}\n",
                    b.width,
                    b.height,
                    b.x_offset,
                    b.y_offset,)
            },
            Property(name, bdf::Property::Str(s)) => {
                write!(f, "{} {}\n", name, s.replace("\"", "\"\""))
            },
            Property(name, bdf::Property::Int(i)) => {
                write!(f, "{} {}\n", name, i)
            },
            BoundingBox(b) => {
                write!(f, "BBX {} {} {} {}\n",
                    b.width,
                    b.height,
                    b.x_offset,
                    b.y_offset,)
            },
            Bitmap(b) => {
                let mut buf = String::from("BITMAP\n");
                buf.reserve(b.width() * b.height() / 4 + b.height());

                for row in b.rows() {
                    for byte in row.to_bytes() {
                        write!(&mut buf, "{:02X}", byte)?;
                    }
                    write!(&mut buf, "\n")?;
                }

                f.write_str(&buf)
            }
        }
    }
}
