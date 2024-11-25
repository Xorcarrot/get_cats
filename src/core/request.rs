use std::env::Args;
use std::fs::File;
use std::io::{copy, Cursor};
use reqwest::Client;
use super::tools::Help;

#[derive(Debug)]
pub struct Image {
    pub path: String,
    pub number: Option<u32>,
    resolution: Option<u16>,
}

impl Image {
    fn create_url(&self) -> String {
        let mut url = "https://api.ai-cats.net/v1/cat?theme=All".to_string();

        if let Some(resolution) = self.resolution {
            url = format!("{url}&size={resolution}");
        }

        url
    }

    pub async fn download_image(&self, client: Client, index: u32) {
        let Ok(response) = client.get(&self.create_url()).send().await else {
            println!("Can not download image");
            return;
        };

        let path = self.path.as_str();
        let file_name = format!("{path}/cat_{index}.jpg");

        let Ok(mut file) = File::create(file_name) else { return; };
        let Ok(bytes) = response.bytes().await else { return; };

        let mut content = Cursor::new(bytes);

        if let Err(_) = copy(&mut content, &mut file) {
            return;
        }
    }

    pub fn new(args: Args) -> Option<Image> {
        let mut path: String = "".to_string();

        let mut number: Option<u32> = None;
        let mut resolution: Option<u16> = None;

        let mut index: usize = 0;
        let args: Vec<String> = args.collect();

        while index < args.len() {
            let arg = args[index].clone();

            if arg.contains("-h") || arg.contains("--help") {
                Help::get_help();
                return None;
            }

            if arg.contains("--res") {
                index += 1;

                let Some(arg) = args.get(index) else {
                    println!("No value found after --res.");
                    continue
                };

                let Ok(arg) = arg.parse() else {
                    println!("Value is no number.");
                    continue;
                };

                let Some(arg) = check_resolution(arg) else {
                    println!("Resolution has no valid number, choose - 16, 32, 64, 128, 256, 512, 1024");
                    continue;
                };

                resolution = Some(arg);

            } else if arg.contains("--sum") {
                index += 1;

                let Some(arg) = args.get(index) else {
                    println!("No value found after --sum.");
                    continue
                };

                let Ok(arg) = arg.parse() else {
                    println!("Value is no number.");
                    continue;
                };

                let Some(arg) = check_sum_is_valid(arg) else {
                    println!("Sum should be smaller than 3000.");
                    continue;
                };

                number = Some(arg);

            } else {
                if arg.starts_with("./") {
                    path = arg;
                } else if arg.starts_with("/") {
                    path = format!(".{arg}");
                } else {
                    path = format!("./{arg}");
                }
            }

            index += 1;
        };

        Some(Image {
            path,
            number,
            resolution,
        })
    }
}

fn check_resolution(res: u16) -> Option<u16> {
    if res == 16 || res == 32 || res == 64 || res == 128 || res == 256 || res == 512 || res == 1024 {
        return Some(res);
    };

    None
}

fn check_sum_is_valid(sum: u32) -> Option<u32> {
    if sum <= 6000 {
        return Some(sum)
    };

    None
}


#[cfg(test)]
mod image_test {
    use super::*;

    #[test]
    fn some_check_resolution() {
        assert!(check_resolution(64).is_some())
    }

    #[test]
    fn none_check_resolution() {
        assert!(check_resolution(234).is_none())
    }

    #[test]
    fn some_check_sum_is_valid() {
        assert!(check_sum_is_valid(435).is_some())
    }

    #[test]
    fn none_check_sum_is_valid() {
        assert!(check_sum_is_valid(7000).is_none())
    }
}