extern crate globwalk;

use std::fs;

fn main() {
    println!("Hello, httpredirect server!");
    for img in globwalk::glob("*.rs").unwrap() {
        if let Ok(file) = img {
            print!("{:?} - ", file.file_name());
            //s::remove_file(img.path()).unwrap();
            let attr = fs::metadata(file.path());
            // inspect attr ...
            println!("{:?}", attr);
        }
    }
}
