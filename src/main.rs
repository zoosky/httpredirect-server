extern crate globwalk;

use std::fs;

fn main() {
    println!("Hello, world!");
    for img in globwalk::glob("*.*").unwrap() {
        if let Ok(file) = img {
            println!("{:?}", file);
            //s::remove_file(img.path()).unwrap();
            let attr = fs::metadata(file.path());
            // inspect attr ...
            println!("{:?}", attr);
        }
    }
}
