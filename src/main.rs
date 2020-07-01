use std::fs::File;
use std::io;
use std::io::prelude::*;

use bdf_font::{
    bdf::Bitmap,
    bdf::Parser,
    bdf::XYPair,
    bdf::ForBdf,
    bdf::BdfBlock,
};

fn main() -> io::Result<()> {
    /*
    let mut bmp = Bitmap::new(10, 10);
    for x in 0..10 {
        for y in 0..10 {
            if x == 1 || y == 1 {
                bmp.set(x, y, true);
            }
        }
    }
    */

    let mut file = File::open("tewi.bdf")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // println!("{}", contents);
    let fnt = Parser::parse(&contents);
    println!("{:#?}", fnt);

    // println!("{}", fnt.unwrap().for_bdf());

    let read = fnt.unwrap().for_bdf().to_string();
    let fnt = Parser::parse(&read);
    println!("{:#?}", fnt);

    Ok(())

    // println!("{}", bmp.for_bdf());
}
