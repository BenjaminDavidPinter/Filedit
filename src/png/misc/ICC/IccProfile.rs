pub struct IccProfile {}

impl IccProfile {
    /*
    The value in the profile size field shall be the exact size obtained by combining the profile
    header, the tag table, and the tagged element data, including the pad bytes for the last tag.
    It shall be encoded as a ulnt32Number.
    */
    pub fn get_profile_size(iccp_profile_bytes: &Vec<u8>) -> u32 {
        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&iccp_profile_bytes[0..4]);
        u32::from_be_bytes(buf)
    }

    /*
    This field may be used to identify the preferred CMM to be used. If used, it shall match a CMM
    type signature registered in the ICC registry (see Clause 5). If no preferred CMM is identified,
    this field shall be set to zero (00000000h).
    */
    pub fn get_profile_preferred_cmm_type(iccp_profile_bytes: &Vec<u8>) -> &[u8] {
        &iccp_profile_bytes[4..8]
    }

    /*
    The profile version with which the profile is compliant shall be encoded as binary-coded decimal
    in the profile version field. The first byte (byte 8) shall identify the major version and byte
    9 shall identify the minor version and bug fix version in each 4-bit half of the byte. Bytes 10
    and 11 are reserved and shall be set to zero. The major and minor versions are set by the
    International Color Consortium. The profile version number consistent with this ICC specification
    is “4.3.0.0” (encoded as 04300000h).
    */
    pub fn get_profile_version(iccp_profile_bytes: &Vec<u8>) -> &[u8] {
        &iccp_profile_bytes[8..12]
    }

    /*
    There are three basic classes of device profiles, which are Input, Display and Output. In
    addition to the three basic device profile classes, four additional colour processing profiles
    are defined. These profiles provide a standard implementation for use by the CMM in general
    colour processing, or for the convenience of CMMs which may use these types to store calculated
    transforms. These four additional profile classes are DeviceLink, ColorSpace, Abstract and NamedColor.

    Table 18 — Profile classes
     __________________________________________________
    | Profile class         | Signature | Hex encoding |
    |-----------------------|-----------|--------------|
    | Input device profile  | ‘scnr’    | 73636E72h    |
    | Display device profile| 'mntr'    | 6D6E7472h    |
    | Output device profile | 'prtr'    | 70727472h    |
    | DeviceLink profile    | 'link'    | 6C696E6Bh    |
    | ColorSpace profile    | 'spac'    | 73706163h    |
    | Abstract profile      | 'abst'    | 61627374h    |
    | NamedColor profile    | 'nmcl'    | 6E6D636Ch    |
    |-----------------------|-----------|--------------|

    */
    pub fn get_profile_class(iccp_profile_bytes: &Vec<u8>) -> &[u8] {
        &iccp_profile_bytes[12..16]
    }

    /*
    This field shall contain the signature of the data colour space expected on the A side (device
    side) of the profile transforms. The names and signatures of the permitted data colour spaces
    are shown in Table 19. Signatures are left justified.
    */
    pub fn get_color_space(iccp_profile_bytes: &Vec<u8>) -> &[u8] {
        &iccp_profile_bytes[16..20]
    }

    /*
    For all profile classes (see Table 18), other than a DeviceLink profile, the PCS encoding shall
    be either PCSXYZ or PCSLAB and the signature shall be as defined in Table 19. When the
    profile/device class is a DeviceLink profile, the value of the PCS shall be the appropriate data
    colour space from Table 19. The field represents the colour space on the B side (PCS side) of
    the transform.
    */
    pub fn get_pcs_encoding(iccp_profile_bytes: &Vec<u8>) -> &[u8] {
        &iccp_profile_bytes[20..24]
    }

    /*
    This header field shall contain the date and time that the profile was first created, encoded
    as a dateTimeNumber.
    */
    pub fn get_date_and_time(iccp_profile_bytes: &Vec<u8>) -> &[u8] {
        &iccp_profile_bytes[24..36]
    }

    /*
    The profile file signature field shall contain the value “acsp” (61637379h) as a profile file
    signature
    */
    pub fn get_file_signature(iccp_profile_bytes: &Vec<u8>) -> &[u8] {
        &iccp_profile_bytes[36..40]
    }

    /*
    This field may be used to identify the primary platform/operating system framework for which the
    profile was created. The primary platforms that have been identified, and the signatures that
    shall be used are shown in Table 20. If there is no primary platform identified, this field
    shall be set to zero (00000000h).
    */
    pub fn get_primary_platform(iccp_profile_bytes: &Vec<u8>) -> &[u8] {
        &iccp_profile_bytes[40..44]
    }

    /*
    The profile flags field shall contain flags to indicate various hints for the CMM such as
    distributed processing and caching options. The least-significant 16 bits are reserved for the
    ICC. Flags in bit positions 0 and 1 shall be used as indicated in Table 21. Annex B describes
    embedding device profiles within EPS, TIFF, JFIF and GIF image files.
    */
    pub fn get_profile_flags(iccp_profile_bytes: &Vec<u8>) -> &[u8] {
        &iccp_profile_bytes[44..48]
    }

    /*
    This field may be used to identify a device manufacturer. If used the signature shall match the
    signature contained in the appropriate section of the ICC signature registry found at
    www.color.org (see Clause 5). If not used this field shall be set to zero (00000000h).
    */
    pub fn get_device_manufacturer(iccp_profile_bytes: &Vec<u8>) -> &[u8] {
        &iccp_profile_bytes[48..52]
    }

    /*
    This field may be used to identify a device model. If used the signature shall match the
    signature contained in the appropriate section of the ICC signature registry found at
    www.color.org (see Clause 5). If not used this field shall be set to zero (00000000h).
    */
    pub fn get_device_model(iccp_profile_bytes: &Vec<u8>) -> &[u8] {
        &iccp_profile_bytes[52..56]
    }

    /*
    The device attributes field shall contain flags used to identify attributes unique to the
    particular device setup for which the profile is applicable. The least-significant 32 bits of
    this 64-bit value are defined by the ICC. Bit usage shall be used as shown in Table 22.
    */
    pub fn get_device_attributes(iccp_profile_bytes: &Vec<u8>) -> &[u8] {
        &iccp_profile_bytes[56..64]
    }

    /*
    The rendering intent field shall specify the rendering intent which should be used (or, in the
    case of a DeviceLink profile, was used) when this profile is (was) combined with another profile.
    In a sequence of more than two profiles, it applies to the combination of this profile and the
    next profile in the sequence and not to the entire sequence. Typically, the user or application
    will set the rendering intent dynamically at runtime or embedding time. Therefore, this flag may
    not have any meaning until the profile is used in some context, e.g. in a DeviceLink or an
    embedded source profile.

    The field is a uInt32Number in which the least-significant 16 bits shall be used to encode the
    rendering intent. The most significant 16 bits shall be set to zero (0000h).

    The defined rendering intents are perceptual, media-relative colorimetric, saturation and
    ICC-absolute colorimetric. These shall be identified using the values shown in Table 23.

    Table 23 — Rendering intents
     ___________________________________
    |Rendering intent           | Value |
    |---------------------------|-------|
    |Perceptual                 | 0     |
    |Media-relative colorimetric| 1     |
    |Saturation                 | 2     |
    |ICC-absolute colorimetric  | 3     |
    |---------------------------|-------|

    */
    pub fn get_rendering_intent(icc_profile_bytes: &Vec<u8>) -> &[u8] {
        &icc_profile_bytes[64..68]
    }

    /*
    The PCS illuminant field shall contain the nCIEXYZ values X = 0,964 2, Y = 1,0 and Z = 0,824 9
        encoded as an XYZNumber. See Annex A for further details.

    NOTE These values are the nCIEXYZ values of CIE illuminant D50
    */
    pub fn get_pcs_illuminant(icc_profile_bytes: &Vec<u8>) -> &[u8] {
        &icc_profile_bytes[68..80]
    }

    /*
    This field may be used to identify the creator of the profile. If used the signature should
    match the signature contained in the device manufacturer section of the ICC signature registry
    found at www.color.org. If not used this field shall be set to zero (00000000h).
    */
    pub fn get_profile_creator(icc_profile_bytes: &Vec<u8>) -> &[u8] {
        &icc_profile_bytes[80..84]
    }

    /*
    This field, if not zero (00h), shall hold the Profile ID. The Profile ID shall be calculated
    using the MD5 fingerprinting method as defined in Internet RFC 1321. The entire profile, whose
    length is given by the size field in the header, with the profile flags field (bytes 44 to 47,
    see 7.2.11), rendering intent field (bytes 64 to 67, see 7.2.15), and profile ID field (bytes
    84 to 99) in the profile header temporarily set to zeros (00h), shall be used to calculate the
    ID. A profile ID field value of zero (00h) shall indicate that a profile ID has not been calculated.

    It is suggested that profile creators compute and record a profile ID.
    */
    pub fn get_profile_id(icc_profile_bytes: &Vec<u8>) -> &[u8] {
        &icc_profile_bytes[84..100]
    }

    /*
    This field of the profile header is reserved for future ICC definition and shall be set to zero.
    */
    pub fn get_reserved_bits(icc_profile_bytes: &Vec<u8>) -> &[u8] {
        &icc_profile_bytes[100..128]
    }
}
