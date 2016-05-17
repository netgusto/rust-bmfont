extern crate bmfont;

use bmfont::{ parse };

fn main() {
    use std::io::prelude::*;
    use std::fs::File;

    let mut f = File::open("examples/assets/font.fnt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    let res = parse(buffer);
    println!("{:?}", res);
}