use std::env;

use argument::core::{request::Image, native::Dictionary};

fn main() {
    let args = env::args();

    let image = Image::new(args);

    Dictionary::new(&image.path);

    println!("{:?}", image)
}
