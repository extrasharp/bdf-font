pub mod bdf;
pub mod entry;
pub mod writer;

//

use std::io::{
    self,
    Write,
};

//

use {
    bdf::{
        Font,
    },
    entry::Entry,
    writer::Writer,
};

//

pub fn write<T: Write>(stream: &mut Writer<T>, font: &Font) -> io::Result<()> {
    stream.write(Entry::StartFont(font.bdf_version.clone()))?;
    stream.write(Entry::Font(font.name.clone()))?;
    if let Some(cv) = &font.content_version {
        stream.write(Entry::ContentVersion(cv.clone()))?;
    }
    stream.write(Entry::Size(font.size))?;
    stream.write(Entry::FontBoundingBox(font.bounding_box))?;
    stream.write(Entry::MetricsSet(font.metrics))?;
    Ok(())
}
