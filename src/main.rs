use std::env;
use std::path::Path;
pub mod png;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = String::new();

    /*
    There is certainly a better way to do this, and we can work on this a little more once we
        begin to introduce more file formats.
    */
    for arg in &args {
        println!("{:?}", arg);
        if arg.ends_with("png") {
            if Path::new(&arg).is_file() {
                file = String::from(arg);
            }
            else {
                panic!("{} is not a parsable file", arg);
            }
            png::parse(&file)
        }
    }
}
