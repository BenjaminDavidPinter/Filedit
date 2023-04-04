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

    pub fn get_profile_size(iccp_profile_bytes: &Vec<u8>) -> u32 {
        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&iccp_profile_bytes[0..4]);
        u32::from_be_bytes(buf)
    }

    pub fn get_profile_preferred_CMM_type(iccp_profile_bytes: &Vec<u8>) -> [u8; 4] {
        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&iccp_profile_bytes[4..8]);
        buf
    }

    pub fn get_profile_version(iccp_profile_bytes: &Vec<u8>) -> [u8; 4] {
        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&iccp_profile_bytes[8..12]);
        buf
    }

    pub fn get_profile_class(iccp_profile_bytes: &Vec<u8>) -> [u8; 4] {
        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&iccp_profile_bytes[12..16]);
        buf
    }

    pub fn get_color_space(iccp_profile_bytes: &Vec<u8>) -> [u8; 4] {
        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&iccp_profile_bytes[16..20]);
        buf
    }

    pub fn get_pcs_encoding(iccp_profile_bytes: &Vec<u8>) -> [u8; 4] {
        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&iccp_profile_bytes[20..24]);
        buf
    }

    pub fn get_date_and_time(iccp_profile_bytes: &Vec<u8>) -> [u8; 12] {
        let mut buf: [u8; 12] = [0; 12];
        buf.copy_from_slice(&iccp_profile_bytes[24..36]);
        buf
    }

    pub fn get_file_signature(iccp_profile_bytes: &Vec<u8>) -> [u8; 4] {
        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&iccp_profile_bytes[36..40]);
        buf
    }
    pub fn get_primary_platform(iccp_profile_bytes: &Vec<u8>) -> [u8; 4] {
        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&iccp_profile_bytes[40..44]);
        buf
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
    println!("Profile Data:");
    println!(
        "\tProfile Size: {:?}",
        iCCP::get_profile_size(&iccp_chunk.get_profile())
    );
    println!(
        "\tPreferred CMM: {:?}",
        String::from_utf8_lossy(&iCCP::get_profile_preferred_CMM_type(
            &iccp_chunk.get_profile()
        ))
    );
    let version = iCCP::get_profile_version(&iccp_chunk.get_profile());
    println!(
        "\tProfile Version: {:?}.{:?}.{:?}.{:?}",
        version[0], version[1], version[2], version[3]
    );
    println!(
        "\tProfile Class: {:?}",
        String::from_utf8_lossy(&iCCP::get_profile_class(&iccp_chunk.get_profile()))
    );
    println!(
        "\tColor Space: {:?}",
        String::from_utf8_lossy(&iCCP::get_color_space(&iccp_chunk.get_profile()))
    );
    println!(
        "\tPCS Encoding: {:?}",
        String::from_utf8_lossy(&iCCP::get_pcs_encoding(&iccp_chunk.get_profile()))
    );
    println!(
        "\tProfile Created On: {:?}",
        interpret_date_and_time(&iCCP::get_date_and_time(&iccp_chunk.get_profile()))
    );
    println!(
        "\tProfile Signature: {:?}",
        String::from_utf8_lossy(&iCCP::get_file_signature(&iccp_chunk.get_profile()))
    );
    println!(
        "\tPrimary Platform: {:?}",
        String::from_utf8_lossy(&iCCP::get_primary_platform(&iccp_chunk.get_profile()))
    );
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

fn interpret_date_and_time(bytes: &[u8; 12]) -> String {
    let mut shared_buf: [u8; 2] = [0; 2];
    shared_buf.copy_from_slice(&bytes[0..2]);
    let year = u16::from_be_bytes(shared_buf);

    shared_buf.copy_from_slice(&bytes[2..4]);
    let month = u16::from_be_bytes(shared_buf);

    shared_buf.copy_from_slice(&bytes[4..6]);
    let day = u16::from_be_bytes(shared_buf);

    shared_buf.copy_from_slice(&bytes[6..8]);
    let hours = u16::from_be_bytes(shared_buf);

    shared_buf.copy_from_slice(&bytes[8..10]);
    let minutes = u16::from_be_bytes(shared_buf);

    shared_buf.copy_from_slice(&bytes[10..12]);
    let seconds = u16::from_be_bytes(shared_buf);

    return String::from(format!(
        "{}/{}/{} {}:{}:{:0>2}",
        month, day, year, hours, minutes, seconds
    ));
}
