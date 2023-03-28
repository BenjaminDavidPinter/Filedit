use crate::png::BaseChunk;

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

#[derive(Debug)]
pub enum ColorType {
    Grayscale,
    Truecolor,
    IndexedColor,
    GrayscaleWithAlpha,
    TruecolorWithAlpha,
}

#[derive(Debug)]
pub enum CompressionType {
    DeflateInflate,
}

#[derive(Debug)]
pub enum FilterMethod {
    Method0,
}

#[derive(Debug)]
pub enum InterfaceMethod {
    Method0,
    Method1,
}
