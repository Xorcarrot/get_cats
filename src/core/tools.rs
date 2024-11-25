//Contains information for the user
pub struct Help;

impl Help {
    pub fn get_help() {
        let command = "cats";
        let path = "file/path";
        let sum = 200;
        let resolution = 256;
        let theme = "Xmas";

        println!("Command Explanation:\n");
        println!("{:<15} - The name of the command to be executed.", command);
        println!("{:<15} - The path where the file should be saved.", path);
        println!("{:<15} - The number of images to process.", format!("--sum {}", sum));
        println!("{:<15} - The desired resolution for the images. Allowed values: 16, 32, 64, 128, 256, 512, 1024.", format!("--res {}", resolution));
        println!("{:<15} - The theme used for the image - Values: All, Xmas, Default, Halloween, Winter", format!("--the {}", theme));

        println!("\nComplete Command:");
        println!("{} {} --sum {} --res {} --the {}", command, path, sum, resolution, theme);
    }
}