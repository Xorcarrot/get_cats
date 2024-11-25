mod core;

use std::env;
use crate::core::native::Directive;
use crate::core::request::Worker;

use std::io::{stdout, Write};

use reqwest::{Client, header::HeaderMap};

#[tokio::main]
async fn main() {
    const ACCEPT: &str = "accept";

    let args = env::args();

    //Header builder
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "image/jpeg".parse().unwrap());

    //Client for requests
    let client = Client::builder().default_headers(headers).build().unwrap();

    //Worker for server calls
    let Some(worker) = Worker::new(args) else {return;};

    //Generate dir
    if let None = Directive::new(&worker.path) {
        println!("Couldn't creat the dictionary");
        return;
    };

    
    let length = worker.number.unwrap_or(1);

    let mut index: u32 = 0;
    print!("{index}/{length}");

    //start downloading images and give user feedback
    while index < length {
        index += 1;
        worker.download_image(client.clone(), index).await;

        let dot_count = index % 3;

        let dots = ".".repeat((dot_count + 1) as usize);
        let spaces = " ".repeat((2 - dot_count) as usize);

        print!("\r{index}/{length} Processing{dots}{spaces}");
        if let Err(_) = stdout().flush() {
            continue;
        }
    }

    println!("\nDownload completed");
}
