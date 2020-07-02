use std::fs::File;
use std::io;
use std::io::prelude::*;

use bdf_font::{
    self,
    BdfBlock,
};

fn main() -> io::Result<()> {
    let mut file = File::open("tewi.bdf")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let fnt = bdf_font::parse_font(&contents);
    println!("{:#?}", fnt);

    let read = fnt.unwrap().for_bdf().unwrap().to_string();
    let fnt = bdf_font::parse_font(&read);
    println!("{:#?}", fnt);
    println!("{}", read);

    Ok(())
}
