use std::{fs, io::Read};

pub mod chunks;
pub mod misc;

#[derive(Debug)]
pub struct BaseChunk {
    pub length: [u8; 4],
    pub ctype: [u8; 4],
    pub data: Vec<u8>,
    pub crc: [u8; 4],
}

impl BaseChunk {
    pub fn get_total_size(&self) -> usize {
        let mut chunk_size: [u8; 4] = [0; 4];
        chunk_size.copy_from_slice(&self.length[0..4]);
        let chunk_size = u32::from_be_bytes(chunk_size);
        usize::try_from(chunk_size).unwrap() + usize::try_from(12).unwrap()
    }
}

pub fn check_png_signature(bytes: &[u8]) -> bool {
    let signature: Vec<u8> = vec![137, 80, 78, 71, 13, 10, 26, 10];
    signature.eq(bytes)
}

pub fn read_png_chunk_from_bytes(bytes: &[u8]) -> BaseChunk {
    let mut chunk_size: [u8; 4] = [0; 4];
    chunk_size.copy_from_slice(&bytes[0..4]);
    let chunk_size = u32::from_be_bytes(chunk_size);
    let chunk_size = usize::try_from(chunk_size).unwrap();

    let mut chunk_length = [0; 4];
    let mut chunk_type = [0; 4];
    let mut chunk_data: Vec<u8> = vec![0; chunk_size];
    let mut chunk_crc = [0; 4];

    chunk_length.copy_from_slice(&bytes[0..4]);
    chunk_type.copy_from_slice(&bytes[4..8]);
    chunk_data.copy_from_slice(&bytes[8..usize::try_from(chunk_size + 8).unwrap()]);
    chunk_crc.copy_from_slice(&bytes[chunk_size..usize::try_from(chunk_size + 4).unwrap()]);

    BaseChunk {
        length: chunk_length,
        ctype: chunk_type,
        data: chunk_data,
        crc: chunk_crc,
    }
}

pub fn parse(file: &str) {
    let mut file = fs::File::open(&file).unwrap();
    let mut file_bytes = vec![];
    let file_size = file.read_to_end(&mut file_bytes).unwrap();

    if !check_png_signature(&file_bytes[0..8]) {
        panic!("Not a valid png");
    }
    let mut offset = 8;

    while offset < file_size {
        let next_chunk = read_png_chunk_from_bytes(&file_bytes[offset..]);
        match next_chunk.ctype {
            chunks::IHDR::CTYPE => {
                let ihdr_chunk = chunks::IHDR::from_base_chunk(&next_chunk);
                chunks::IHDR::print_chunk(&ihdr_chunk);
            }
            chunks::iCCP::CTYPE => {
                let iccp_chunk = chunks::iCCP::from_base_chunk(&next_chunk);
                chunks::iCCP::print_chunk(&iccp_chunk);
            }
            _ => {
                println!("===={:?}====", String::from_utf8_lossy(&next_chunk.ctype));
            }
        }

        offset += next_chunk.get_total_size();
    }
}
