use std::env;
use std::{fs, io::Read};
mod png;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = fs::File::open(&args[1]).unwrap();
    let mut file_bytes = vec![];
    let file_size = file.read_to_end(&mut file_bytes).unwrap();

    if !png::check_png_signature(&file_bytes[0..8]) {
        panic!("Not a valid png");
    }

    let mut offset = 8;

    while offset < file_size {
        let next_chunk = png::read_png_chunk_from_bytes(&file_bytes[offset..]);
        println!("===={:?}====", String::from_utf8_lossy(&next_chunk.ctype));
        offset += next_chunk.get_total_size();
    }
}
