use std::env;
use std::{any::Any, fs, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = fs::File::open(&args[1]).unwrap();
    let mut file_bytes = vec![];
    let file_size = file.read_to_end(&mut file_bytes).unwrap();
    println!("Read {:?} bytes", file_size);
    println!(
        "Valid PNG Signature: {:?}",
        //First 8 bytes of a png file are the same.
        check_png_signature(&file_bytes[0..8])
    );
    let mut offset = usize::try_from(8).unwrap();
    while (offset < file_size) {
        let next_chunk = read_next_chunk(&file_bytes[offset..]);
        let mut chunk_size: [u8; 4] = [0; 4];
        chunk_size.copy_from_slice(&next_chunk.Length[0..4]);
        let chunk_size = u32::from_be_bytes(chunk_size);
        println!("===={:?}====", String::from_utf8_lossy(&next_chunk.Type));
        offset += usize::try_from(chunk_size).unwrap() + usize::try_from(12).unwrap();
    }
}

fn check_png_signature(bytes: &[u8]) -> bool {
    let signature: Vec<u8> = vec![137, 80, 78, 71, 13, 10, 26, 10];
    return signature.eq(bytes);
}

fn read_next_chunk(bytes: &[u8]) -> PngChunk {
    let mut chunk_size: [u8; 4] = [0; 4];
    chunk_size.copy_from_slice(&bytes[0..4]);
    let chunk_size = u32::from_be_bytes(chunk_size);
    let chunk_size = usize::try_from(chunk_size).unwrap();

    PngChunk {
        Length: &bytes[0..4],
        Type: &bytes[4..8],
        Data: &bytes[8..usize::try_from(chunk_size + 8).unwrap()],
        Crc: &bytes[chunk_size..usize::try_from(chunk_size + 4).unwrap()],
    }
}

#[derive(Debug)]
struct PngChunk<'parent> {
    pub Length: &'parent [u8],
    pub Type: &'parent [u8],
    pub Data: &'parent [u8],
    pub Crc: &'parent [u8],
}
