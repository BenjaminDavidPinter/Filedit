use flate2::read::ZlibDecoder;
use std::io;
use std::io::prelude::*;
use std::str;

use crate::png::BaseChunk;

pub const CTYPE: [u8; 4] = [105, 67, 67, 80];

pub struct iCCP {
    pub length: [u8; 4],
    pub ctype: [u8; 4],
    pub data: Vec<u8>,
    pub crc: [u8; 4],
}

impl iCCP {
    pub fn get_profile_name(&self) -> Vec<u8> {
        self.data
            .clone()
            .into_iter()
            .take_while(|x| !char::is_control(*x as char))
            .collect::<Vec<_>>()
    }

    pub fn get_compression_method(&self) -> CompressionMethod {
        CompressionMethod::Method0
    }

    pub fn get_profile(&self) -> Vec<u8> {
        let total_length = self.get_profile_name().len() + 2;
        decode_reader(self.data[total_length..].to_vec()).unwrap()
    }
}

pub fn from_base_chunk(base_chunk: &BaseChunk) -> iCCP {
    iCCP {
        length: base_chunk.length,
        ctype: base_chunk.ctype,
        data: base_chunk.data.clone(),
        crc: base_chunk.crc,
    }
}

pub fn print_chunk(iccp_chunk: &iCCP) {
    println!("===={:?}====", String::from_utf8_lossy(&iccp_chunk.ctype));
    println!(
        "Profile Name: {:?}",
        String::from(str::from_utf8(&iccp_chunk.get_profile_name()).unwrap())
    );
    println!(
        "Compression Method: {:?}",
        iccp_chunk.get_compression_method()
    );
    println!("Profile Data: {:?}", iccp_chunk.get_profile());
}

#[derive(Debug)]
pub enum CompressionMethod {
    Method0,
}

fn decode_reader(bytes: Vec<u8>) -> io::Result<Vec<u8>> {
    let mut z = ZlibDecoder::new(&bytes[..]);
    let mut s = vec![];
    z.read_to_end(&mut s).unwrap();
    Ok(s)
}
