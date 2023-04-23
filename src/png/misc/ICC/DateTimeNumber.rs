/// Houses ICC Standard Date Time field
///
/// See [ICC.1:2010](https://www.color.org/specification/ICC1v43_2010-12.pdf)
///
/// This struct is not meant to be used anywhere except when reading an iCCP chunk in a PNG file.
/// The iCCP chunk contains a compressed ICC Profile, which contains these fields. It should share the
/// lifetime of the IccProfile struct.
pub struct DateTimeNumber<'profile> {
    pub year: &'profile [u8],
}
