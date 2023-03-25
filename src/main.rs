use std::{fs, io::Read};

fn main() {
    let mut file = fs::File::open("/Users/benjaminpinter/Desktop/SS.png").unwrap();
    let mut file_bytes = vec![];
    let file_size = file.read_to_end(&mut file_bytes).unwrap();
    println!("Read {:?} bytes", file_size);
    println!(
        "Valid PNG Signature: {:?}",
        //First 8 bytes of a png file are the same.
        check_png_signature(&file_bytes[0..8])
    );
}

fn check_png_signature(bytes: &[u8]) -> bool {
    let signature: Vec<u8> = vec![137, 80, 78, 71, 13, 10, 26, 10];
    return signature.eq(bytes);
}
