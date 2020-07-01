use std::fs::File;
use std::io;
use std::io::prelude::*;

use bdf_font::{
    bdf,
    bdf::BdfBlock,
};

fn main() -> io::Result<()> {
    let mut file = File::open("tewi.bdf")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let fnt = bdf::parse_font(&contents);
    println!("{:#?}", fnt);

    let read = fnt.unwrap().for_bdf().unwrap().to_string();
    let fnt = bdf::parse_font(&read);
    println!("{:#?}", fnt);

    Ok(())
}
