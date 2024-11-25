use std::fs;

pub struct Directive;

impl Directive {
    pub fn new(path: &str) -> Option<()> {
        let exists = fs::exists(path).unwrap_or(false);

        if exists == false {
            let Ok(()) = fs::create_dir_all(path) else {
                println!("Failed to create dictionary.");
                return None
            };
        }

        Some(())
    }
}