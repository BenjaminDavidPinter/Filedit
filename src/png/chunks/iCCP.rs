use flate2::read::ZlibDecoder;
use std::io;
use std::io::prelude::*;

use crate::png::misc::ICC::IccProfile::IccProfile;
use crate::png::BaseChunk;

pub const CTYPE: [u8; 4] = [105, 67, 67, 80];

pub struct iCCP<'a> {
    pub length: &'a [u8],
    pub ctype: &'a [u8],
    pub data: &'a [u8],
    pub crc: &'a [u8],
}

impl iCCP<'_> {
    //TODO: Rewrite this, I don't want to copy the profile data.
    pub fn get_profile_name(&self) -> &[u8] {
        /*Find the first index of control character, and then use that to perform a standard slice*/
        let position_of_stop = self
            .data
            .iter()
            .position(|&x| char::is_control(x as char))
            .unwrap();

        &self.data[0..position_of_stop]
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
        length: &base_chunk.length,
        ctype: &base_chunk.ctype,
        data: &base_chunk.data,
        crc: &base_chunk.crc,
    }
}

pub fn print_chunk(iccp_chunk: &iCCP) {
    println!("===={:?}====", String::from_utf8_lossy(&iccp_chunk.ctype));
    println!(
        "Profile Name: {:?}",
        String::from_utf8_lossy(&iccp_chunk.get_profile_name())
    );
    println!(
        "Compression Method: {:?}",
        iccp_chunk.get_compression_method()
    );
    println!("Profile Data:");
    println!(
        "\tProfile Size: {:?}",
        IccProfile::get_profile_size(&iccp_chunk.get_profile())
    );
    println!(
        "\tPreferred CMM: {:?}",
        String::from_utf8_lossy(&IccProfile::get_profile_preferred_cmm_type(
            &iccp_chunk.get_profile()
        ))
    );
    let profile_data = iccp_chunk.get_profile();
    let version = IccProfile::get_profile_version(&profile_data);
    println!(
        "\tProfile Version: {:?}.{:?}.{:?}.{:?}",
        version[0], version[1], version[2], version[3]
    );
    println!(
        "\tProfile Class: {:?}",
        String::from_utf8_lossy(IccProfile::get_profile_class(&profile_data))
    );
    println!(
        "\tColor Space: {:?}",
        String::from_utf8_lossy(IccProfile::get_color_space(&profile_data))
    );
    println!(
        "\tPCS Encoding: {:?}",
        String::from_utf8_lossy(IccProfile::get_pcs_encoding(&profile_data))
    );
    println!(
        "\tProfile Created On: {:?}",
        interpret_date_and_time(IccProfile::get_date_and_time(&profile_data))
    );
    println!(
        "\tProfile Signature: {:?}",
        String::from_utf8_lossy(IccProfile::get_file_signature(&profile_data))
    );
    println!(
        "\tPrimary Platform: {:?}",
        String::from_utf8_lossy(IccProfile::get_primary_platform(&profile_data))
    );
    println!(
        "\tProfile Flags: {:?}",
        IccProfile::get_profile_flags(&profile_data)
    );
    println!(
        "\tDevice Manufacturer: {:?}",
        IccProfile::get_device_manufacturer(&profile_data)
    );
    println!(
        "\tDevice Model: {:?}",
        IccProfile::get_device_model(&profile_data)
    );
    println!(
        "\tDevice Attrs: {:?}",
        IccProfile::get_device_attributes(&profile_data)
    );
    println!(
        "\tRendering Intent: {:?}",
        IccProfile::get_rendering_intent(&profile_data)
    );
    println!(
        "\tPCS Illuminant: {:?}",
        IccProfile::get_pcs_illuminant(&profile_data)
    );
    println!(
        "\tProfile Creator: {:?}",
        IccProfile::get_profile_creator(&profile_data)
    );
    println!(
        "\tProfile ID: {:?}",
        IccProfile::get_profile_id(&profile_data)
    );
    println!(
        "\tResrved Bits: {:?}",
        IccProfile::get_reserved_bits(&profile_data)
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

//TODO: Move this into DateTimeNumber
fn interpret_date_and_time(bytes: &[u8]) -> String {
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
