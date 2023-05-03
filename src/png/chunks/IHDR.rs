use crate::png::BaseChunk;
pub const CTYPE: [u8; 4] = [73, 72, 68, 82];

pub struct IHDR {
    pub length: [u8; 4],
    pub ctype: [u8; 4],
    pub data: [u8; 13],
    pub crc: [u8; 4],
}

impl IHDR {
    pub fn get_width(&self) -> u32 {
        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&self.data[0..4]);
        u32::from_be_bytes(buf)
    }

    pub fn get_height(&self) -> u32 {
        let mut buf: [u8; 4] = [0; 4];
        buf.copy_from_slice(&self.data[4..8]);
        u32::from_be_bytes(buf)
    }

    pub fn get_bit_depth(&self) -> u8 {
        self.data[8]
    }

    pub fn get_color_type(&self) -> ColorType {
        match self.data[9] {
            0 => ColorType::Grayscale,
            2 => ColorType::Truecolor,
            3 => ColorType::IndexedColor,
            4 => ColorType::GrayscaleWithAlpha,
            6 => ColorType::TruecolorWithAlpha,
            _ => panic!("Invalid or unknown color type"),
        }
    }

    pub fn get_compression_method(&self) -> CompressionType {
        match self.data[10] {
            0 => CompressionType::DeflateInflate,
            _ => panic!("Unknown compression method"),
        }
    }

    pub fn get_filter_method(&self) -> FilterMethod {
        match self.data[11] {
            0 => FilterMethod::Method0,
            _ => panic!("Unknown filter method"),
        }
    }

    pub fn get_interface_method(&self) -> InterfaceMethod {
        match self.data[12] {
            0 => InterfaceMethod::Method0,
            1 => InterfaceMethod::Method1,
            _ => panic!("Unknown interface method"),
        }
    }
}

pub fn from_base_chunk(base_chunk: &BaseChunk) -> IHDR {
    let mut buf: [u8; 13] = [0; 13];
    buf.copy_from_slice(&base_chunk.data[0..13]);

    IHDR {
        length: base_chunk.length,
        ctype: base_chunk.ctype,
        data: buf,
        crc: base_chunk.crc,
    }
}

pub fn print_chunk(ihdr_chunk: &IHDR) {
    println!("{:=^50}", String::from_utf8_lossy(&ihdr_chunk.ctype));
    println!("{:^23} = {:^25}", "Field", "Value");
    println!("{:=^50}","=");
    println!("{:>23} = {}", "Width", ihdr_chunk.get_width());
    println!("{:>23} = {}", "Height", ihdr_chunk.get_height());
    println!("{:>23} = {}", "Bit Depth", ihdr_chunk.get_bit_depth());
    println!("{:>23} = {}", "Color Type", ihdr_chunk.get_color_type());
    println!("{:>23} = {}", "Compression Mthd.", ihdr_chunk.get_compression_method());
    println!("{:>23} = {}", "Filter Mthd.", ihdr_chunk.get_filter_method());
    println!("{:>23} = {}", "Interface Mthd.", ihdr_chunk.get_interface_method());
    println!("{:=^50}","=");
    println!();
}

#[derive(Debug)]
pub enum ColorType {
    Grayscale,
    Truecolor,
    IndexedColor,
    GrayscaleWithAlpha,
    TruecolorWithAlpha,
}

impl std::fmt::Display for ColorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match *self {
            ColorType::Grayscale => "Grayscale",
            ColorType::Truecolor => "True Color",
            ColorType::IndexedColor => "Indexed Color",
            ColorType::GrayscaleWithAlpha  => "Grayscale w/Alpha",
            ColorType::TruecolorWithAlpha => "True Color w/Alpha"
        };
        write!(f, "{}", printable)
    }
}

#[derive(Debug)]
pub enum CompressionType {
    DeflateInflate,
}

impl std::fmt::Display for CompressionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match *self {
            CompressionType::DeflateInflate => "Deflate/Inflate"
        };
        write!(f, "{}", printable)
    }
}

#[derive(Debug)]
pub enum FilterMethod {
    Method0,
}

impl std::fmt::Display for FilterMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match *self {
            FilterMethod::Method0 => "Method 0"
        };
        write!(f, "{}", printable)
    }
}

#[derive(Debug)]
pub enum InterfaceMethod {
    Method0,
    Method1,
}

impl std::fmt::Display for InterfaceMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match *self {
            InterfaceMethod::Method0 => "Method 0",
            InterfaceMethod::Method1 => "Method 1"
        };
        write!(f, "{}", printable)
    }
}