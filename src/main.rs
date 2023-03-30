use std::env;
use std::{fs, io::Read};
pub mod png;

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
        match next_chunk.ctype {
            png::chunks::IHDR::CTYPE => {
                let ihdr_chunk = png::chunks::IHDR::from_base_chunk(&next_chunk);
                png::chunks::IHDR::print_chunk(&ihdr_chunk);
            }
            png::chunks::iCCP::CTYPE => {
                let iccp_chunk = png::chunks::iCCP::from_base_chunk(&next_chunk);
                png::chunks::iCCP::print_chunk(&iccp_chunk);
            }
            _ => {
                println!("===={:?}====", String::from_utf8_lossy(&next_chunk.ctype));
            }
        }

        offset += next_chunk.get_total_size();
    }
}
