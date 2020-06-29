use std::{
    io::{
        self,
        Write,
        BufWriter,
    },
};

//

use crate::{
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

    pub fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }

    pub fn into_inner(self) -> Result<T, io::IntoInnerError<BufWriter<T>>> {
        self.0.into_inner()
    }
}
