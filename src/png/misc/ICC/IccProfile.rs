use crate::png::chunks::iccp;
use crate::png::misc::ICC::DateTimeNumber::DateTimeNumber;

pub struct IccProfile<'a> {
    pub name: Vec<&'a u8>,
    pub version: &'a [u8],
    pub class: &'a [u8],
    pub color_space: &'a [u8],
    pub pcs_encoding: &'a [u8],
    pub date_and_time: &'a DateTimeNumber<'a>,
    pub file_signature: &'a [u8],
    pub primary_platform: &'a [u8],
    pub profile_size: &'a u32,
}

impl<'a> IccProfile<'a> {
    pub fn new(from_chunk: &iccp::iCCP) -> IccProfile<'a> {
        let profile_data = from_chunk.get_profile();
        IccProfile {
            name: from_chunk.get_profile_name(),
            version: IccProfile::get_profile_version(&profile_data),
            class: IccProfile::get_profile_class(&profile_data),
            color_space: IccProfile::get_color_space(&profile_data),
            pcs_encoding: IccProfile::get_pcs_encoding(&profile_data),
            date_and_time: todo!("Implement full datetime parser"),
            file_signature: IccProfile::get_file_signature(&profile_data),
            primary_platform: IccProfile::get_primary_platform(&profile_data),
            profile_size: &IccProfile::get_profile_size(&profile_data),
        }
    }

    pub fn get_profile_size(iccp_profile_bytes: &Vec<u8>) -> u32 {
        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&iccp_profile_bytes[0..4]);
        u32::from_be_bytes(buf)
    }

    pub fn get_profile_preferred_cmm_type(iccp_profile_bytes: &Vec<u8>) -> &[u8] {
        &iccp_profile_bytes[4..8]
    }

    pub fn get_profile_version(iccp_profile_bytes: &Vec<u8>) -> &[u8] {
        &iccp_profile_bytes[8..12]
    }

    pub fn get_profile_class(iccp_profile_bytes: &Vec<u8>) -> &[u8] {
        &iccp_profile_bytes[12..16]
    }

    pub fn get_color_space(iccp_profile_bytes: &Vec<u8>) -> &[u8] {
        &iccp_profile_bytes[16..20]
    }

    pub fn get_pcs_encoding(iccp_profile_bytes: &Vec<u8>) -> &[u8] {
        &iccp_profile_bytes[20..24]
    }

    pub fn get_date_and_time(iccp_profile_bytes: &Vec<u8>) -> &[u8] {
        &iccp_profile_bytes[24..36]
    }

    pub fn get_file_signature(iccp_profile_bytes: &Vec<u8>) -> &[u8] {
        &iccp_profile_bytes[36..40]
    }

    pub fn get_primary_platform(iccp_profile_bytes: &Vec<u8>) -> &[u8] {
        &iccp_profile_bytes[40..44]
    }
}
