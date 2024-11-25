mod core;

use std::env;
use crate::core::native::Directive;
use crate::core::request::Image;

use reqwest::{Client, header::HeaderMap};

#[tokio::main]
async fn main() {
    const ACCEPT: &str = "accept";

    let args = env::args();

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "image/jpeg".parse().unwrap());

    let client = Client::builder().default_headers(headers).build().unwrap();

    let Some(image) = Image::new(args) else {return;};

    if let None = Directive::new(&image.path) {
        println!("Couldn't creat the dictionary");
        return;
    };

    let length = image.number.unwrap_or(1);

    let mut index: u32 = 0;
    print!("{index}/{length}");

    while index < length {
        index += 1;
        image.download_image(client.clone(), index).await;

        print!("\r{index}/{length}");
    }

    println!("\nDownload completed");
}
