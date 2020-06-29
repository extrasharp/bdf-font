use std::{
    io::{
        self,
        Write,
        BufWriter,
    },
};

//

use crate::{
    bdf::{
        Font,
        Glyph,
    },
    entry::Entry,
};

//

pub struct Writer<T: Write>(BufWriter<T>);

impl<T: Write> Writer<T> {
    pub fn new(stream: T) -> Self {
        // TODO think abt buffer capacity
        Self(BufWriter::new(stream))
    }

    pub fn write(&mut self, entry: Entry) -> io::Result<()> {
        self.0.write(entry.to_string().as_bytes())?;
        Ok(())
    }

    fn write_glyph(&mut self, glyph: &Glyph) -> io::Result<()> {
        self.write(Entry::StartChar(glyph.name.clone()))?;
        self.write(Entry::Encoding(glyph.codepoint))?;
        self.write(Entry::BoundingBox(glyph.bounding_box))?;
        // TODO only do if metrics != normal ?
        self.write(Entry::MetricsSet(glyph.metrics))?;
        if let &Some(pair) = &glyph.scalable_width {
            self.write(Entry::ScalableWidth(pair))?;
        }
        if let &Some(pair) = &glyph.device_width {
            self.write(Entry::ScalableWidth(pair))?;
        }
        if let &Some(pair) = &glyph.scalable_width_alt {
            self.write(Entry::ScalableWidthAlt(pair))?;
        }
        if let &Some(pair) = &glyph.device_width_alt {
            self.write(Entry::DeviceWidthAlt(pair))?;
        }
        if let &Some(pair) = &glyph.vector {
            self.write(Entry::Vector(pair))?;
        }
        // TODO really want to clone here?
        self.write(Entry::Bitmap(glyph.bitmap.clone()))?;
        self.write(Entry::EndChar)
    }

    pub fn write_font(&mut self, font: &Font) -> io::Result<()> {
        // TODO verify font ?
        self.write(Entry::StartFont(font.bdf_version.clone()))?;
        self.write(Entry::Font(font.name.clone()))?;
        for comment in &font.comments {
            self.write(Entry::Comment(comment.clone()))?;
        }
        if let Some(cv) = &font.content_version {
            self.write(Entry::ContentVersion(cv.clone()))?;
        }
        self.write(Entry::Size(font.size))?;
        self.write(Entry::FontBoundingBox(font.bounding_box))?;
        // TODO only do if metrics != normal ?
        self.write(Entry::MetricsSet(font.metrics))?;
        if let &Some(pair) = &font.scalable_width {
            self.write(Entry::ScalableWidth(pair))?;
        }
        if let &Some(pair) = &font.device_width {
            self.write(Entry::ScalableWidth(pair))?;
        }
        if let &Some(pair) = &font.scalable_width_alt {
            self.write(Entry::ScalableWidthAlt(pair))?;
        }
        if let &Some(pair) = &font.device_width_alt {
            self.write(Entry::DeviceWidthAlt(pair))?;
        }
        if let &Some(pair) = &font.vector {
            self.write(Entry::Vector(pair))?;
        }
        if font.properties.len() > 0 {
            self.write(Entry::StartProperties(font.properties.len()))?;
            for property in &font.properties {
                self.write(Entry::Property(property.clone()))?;
            }
            self.write(Entry::EndProperties)?;
        }
        if font.glyphs.len() > 0 {
            self.write(Entry::Chars(font.glyphs.len()))?;
            for glyph in &font.glyphs {
                self.write_glyph(glyph)?;
            }
        }

        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }

    pub fn into_inner(self) -> Result<T, io::IntoInnerError<BufWriter<T>>> {
        self.0.into_inner()
    }
}
