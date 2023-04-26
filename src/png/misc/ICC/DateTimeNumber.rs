/// Houses ICC Standard Date Time field
///
/// See [ICC.1:2010](https://www.color.org/specification/ICC1v43_2010-12.pdf)
///
/// This struct is not meant to be used anywhere except when reading an iCCP chunk in a PNG file.
/// The iCCP chunk contains a compressed ICC Profile, which contains these fields. It should share the
/// lifetime of the IccProfile struct.
#[derive(Debug)]
pub struct DateTimeNumber<'profile> {
    pub year: &'profile [u8],
    pub month: &'profile [u8],
    pub day: &'profile [u8],
    pub hours: &'profile [u8],
    pub minutes: &'profile [u8],
    pub seconds: &'profile [u8],
}

impl<'profile> DateTimeNumber<'profile> {
    pub fn new(bytes: &[u8]) -> DateTimeNumber {
        DateTimeNumber {
            year: &bytes[0..2],
            month: &bytes[2..4],
            day: &bytes[4..6],
            hours: &bytes[6..8],
            minutes: &bytes[8..10],
            seconds: &bytes[10..12],
        }
    }
}
