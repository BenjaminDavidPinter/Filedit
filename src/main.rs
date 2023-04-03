use std::env;
use std::path::Path;
pub mod png;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut operation = 0;
    let mut file = String::new();

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

    println!("Doing operation {} on {}", operation, file);

    match operation {
        1 => png::parse(&file),
        _ => panic!("Invalid operation"),
    }
}
