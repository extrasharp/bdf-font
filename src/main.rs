use bdf_font::{
    entry::Entry,
    writer::Writer,
};

fn main() {
    let s = Vec::<u8>::new();
    let mut stream = Writer::new(s);
    stream.write(Entry::StartFont(String::from("2.2"))).unwrap();
    stream.write(Entry::Comment(String::from("comment"))).unwrap();
    stream.write(Entry::Comment(String::from("comment2"))).unwrap();
    stream.write(Entry::ScalableWidth(10, 15)).unwrap();
    stream.flush().unwrap();

    println!("{}", String::from_utf8(stream.into_inner().unwrap()).unwrap());
}
