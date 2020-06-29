use bdf_font::{
    entry::Entry,
    writer::Writer,
    bdf::Bitmap,
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

    let s = Vec::<u8>::new();
    let mut stream = Writer::new(s);
    stream.write(Entry::StartFont(String::from("2.2"))).unwrap();
    stream.write(Entry::Comment(String::from("comment"))).unwrap();
    stream.write(Entry::Comment(String::from("comment2"))).unwrap();
    stream.write(Entry::ScalableWidth(10, 15)).unwrap();
    stream.write(Entry::Bitmap(bmp)).unwrap();
    stream.flush().unwrap();

    println!("{}", String::from_utf8(stream.into_inner().unwrap()).unwrap());
}
