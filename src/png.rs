#[derive(Debug)]
pub struct Png<'parent> {
    pub chunks: &'parent [PngChunk<'parent>],
}

impl<'parent> Png<'_> {
    pub fn New(from_chunks: &'parent [PngChunk<'parent>]) -> Png<'parent> {
        Png {
            chunks: from_chunks,
        }
    }
}

#[derive(Debug)]
pub struct PngChunk<'parent> {
    pub length: &'parent [u8],
    pub chunk_type: &'parent [u8],
    pub data: &'parent [u8],
    pub crc: &'parent [u8],
}

impl<'parent> PngChunk<'_> {
    pub fn get_total_size(&self) -> usize {
        let mut chunk_size: [u8; 4] = [0; 4];
        chunk_size.copy_from_slice(&self.length[0..4]);
        let chunk_size = u32::from_be_bytes(chunk_size);
        return usize::try_from(chunk_size).unwrap() + usize::try_from(12).unwrap();
    }
}

pub fn check_png_signature(bytes: &[u8]) -> bool {
    let signature: Vec<u8> = vec![137, 80, 78, 71, 13, 10, 26, 10];
    signature.eq(bytes)
}

pub fn read_png_chunk_from_bytes(bytes: &[u8]) -> PngChunk {
    let mut chunk_size: [u8; 4] = [0; 4];
    chunk_size.copy_from_slice(&bytes[0..4]);
    let chunk_size = u32::from_be_bytes(chunk_size);
    let chunk_size = usize::try_from(chunk_size).unwrap();

    PngChunk {
        length: &bytes[0..4],
        chunk_type: &bytes[4..8],
        data: &bytes[8..usize::try_from(chunk_size + 8).unwrap()],
        crc: &bytes[chunk_size..usize::try_from(chunk_size + 4).unwrap()],
    }
}
