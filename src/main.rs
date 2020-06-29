use bdf_font::{
    bdf::BdfElement,
    bdf::Bitmap,
    bdf::XYPair,
};

fn main() {
    let mut bmp = Bitmap::new(10, 10);
    for x in 0..10 {
        for y in 0..10 {
            if x == 1 || y == 1 {
                bmp.set(x, y, true);
            }
        }
    }

    println!("{}", bmp.for_bdf());
}
