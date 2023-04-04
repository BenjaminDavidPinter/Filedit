use std::env;
use std::path::Path;
pub mod png;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut operation = 0;
    let mut file = String::new();

    /*
    There is certainly a better way to do this, and we can work on this a little more once we
        begin to introduce more file formats.
    */
    for arg in &args {
        println!("{:?}", arg);
        if arg.starts_with("-") {
            if arg.contains("png") {
                operation = 1;
            }
        }
        if Path::new(&arg).is_file() {
            file = String::from(arg);
        }
    }

    match operation {
        1 => png::parse(&file),
        _ => panic!("Invalid operation"),
    }
}
