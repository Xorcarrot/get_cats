use std::fs;

pub struct Dictionary {
    path: String,
}

impl Dictionary {
    pub fn new(path: &str) -> Option<Dictionary> {
        let exists = fs::exists(path).unwrap_or(false);

        if exists == false {
            let Ok(()) = fs::create_dir_all(path) else {
                println!("Failed to create dictionary.");
                return None
            };
        }

        Some(
            Dictionary {
                path: path.to_string()
            }
        )
    }
}