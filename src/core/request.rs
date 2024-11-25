use std::env::Args;
use std::fs::File;
use std::io::{copy, Cursor};
use reqwest::Client;
use super::tools::Help;

#[derive(Debug)]
pub struct Worker {
    pub path: String,
    pub number: Option<u32>,
    resolution: Option<u16>,
    query: Option<String>,
}

impl Worker {
    fn create_url(&self) -> String {
        let query = self.query.clone().unwrap_or("All".to_string());
        let mut url = format!("https://api.ai-cats.net/v1/cat?theme={query}");

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

    pub fn new(args: Args) -> Option<Worker> {
        let mut path: String = "".to_string();

        let mut number: Option<u32> = None;
        let mut resolution: Option<u16> = None;
        let mut query: Option<String> = None;

        let mut index: usize = 0;
        let args: Vec<String> = args.collect();

        while index < args.len() {
            let arg = args[index].clone();

            //Prints help for the User
            if arg.contains("-h") || arg.contains("--help") {
                Help::get_help();
                return None;
            }
            //Collects the optional selected theme
            else if arg.contains("--thm") {
                index += 1;

                let Some(arg) = args.get(index) else {
                    println!("No value found after --thm.");
                    continue
                };

                query = Some(arg.to_string());
            }
            //Collects the optional selected resolution
            else if arg.contains("--res") {
                index += 1;

                let Ok(arg) = args.get(index)?.parse() else {
                    println!("No value found or not a number after --res.");
                    continue
                };

                let Some(arg) = check_resolution(arg) else {
                    println!("Resolution has no valid number, choose - 16, 32, 64, 128, 256, 512, 1024");
                    continue;
                };

                resolution = Some(arg);

            }
            //Collects the optional number of images
            else if arg.contains("--sum") {
                index += 1;

                let Ok(arg) = args.get(index)?.parse() else {
                    println!("No value found or not a number after --sum.");
                    continue
                };

                number = Some(arg);

            }
            //Collects the selected path from the args
            else {
                if index <= 1 {
                    if arg.starts_with("./") || arg.starts_with("C:") || arg.starts_with("~") {
                        path = arg;
                    } else if arg.starts_with("/") {
                        path = format!(".{arg}");
                    } else {
                        path = format!("./{arg}");
                    }
                }
            }

            index += 1;
        };

        Some(Worker {
            path,
            number,
            resolution,
            query
        })
    }
}

//Resolution has to be one of the preset values
fn check_resolution(res: u16) -> Option<u16> {
    if res == 16 || res == 32 || res == 64 || res == 128 || res == 256 || res == 512 || res == 1024 {
        return Some(res);
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
}