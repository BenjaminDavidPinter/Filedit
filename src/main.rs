use std::env;
use std::{fs, io::Read};
mod IHDR;
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
        match next_chunk.ctype {
            [73, 72, 68, 82] => {
                let ihdr_chunk = IHDR::from_base_chunk(&next_chunk);
                println!("===={:?}====", String::from_utf8_lossy(&next_chunk.ctype));
                println!("Width: {}", ihdr_chunk.get_width());
                println!("Height: {}", ihdr_chunk.get_height());
                println!("Bit Depth: {}", ihdr_chunk.get_bit_depth());
                println!("Color Type: {:?}", ihdr_chunk.get_color_type());
                println!(
                    "Compression Method: {:?}",
                    ihdr_chunk.get_compression_method()
                );
                println!("Filter Method: {:?}", ihdr_chunk.get_filter_method());
                println!("Interface Method: {:?}", ihdr_chunk.get_interface_method());
            }
            _ => {
                println!("===={:?}====", String::from_utf8_lossy(&next_chunk.ctype));
            }
        }

        offset += next_chunk.get_total_size();
    }
}
