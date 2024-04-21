use std::{fs, path::Path};

fn main() {
    match fs::create_dir("./files"){
        Err(err) => {
            println!("error on creating directory: {}", err);
        },
        _ => {},
    }
    let path = Path::new("./files").join("target.txt");
    let content = "hello rust";
    let res= fs::write(path, content);
    match res {
        Err(err) => {
            println!("error on writing file: {}", err);
        },
        _ => {},
    }
}
