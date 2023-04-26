pub struct IccProfile {}

impl IccProfile {
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
