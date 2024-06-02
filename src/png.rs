use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

struct PNG {}

pub struct PNGFileBuffer {
    buffer: BufReader<File>,
}

impl PNGFileBuffer {
    const SIGNATURE: &'static [u8] = &[137, 80, 78, 71, 13, 10, 26, 10];

    pub fn new(file: File) -> Self {
        let mut buff = BufReader::new(file);
        Self::check_file_signature(&mut buff);
        PNGFileBuffer { buffer: buff }
    }

    // Read the first 8 bytes in and check its a PNG file
    fn check_file_signature<'a>(buff: &mut BufReader<File>) {
        let signature_buffer: &mut [u8] = &mut [0; 8];
        buff.read_exact(signature_buffer).unwrap();
        assert_eq!(signature_buffer, Self::SIGNATURE);
    }

    /// Read one chunk in. Returns None if EOF reached
    pub fn read_chunk(&mut self) -> Option<Chunk> {
        let mut size_bytes: [u8; 4] = [0; 4];
        let _ = self.try_read_exact(&mut size_bytes)?.unwrap();
        let size: u32 = u32::from_be_bytes(size_bytes);

        let mut chunk_type: [u8; 4] = [0; 4];
        let _ = self.try_read_exact(&mut chunk_type)?.unwrap();

        let mut chunk_data: Vec<u8> = vec![0; size as usize];
        let _ = self.try_read_exact(&mut chunk_data)?.unwrap();

        let mut crc: [u8; 4] = [0; 4];
        let _ = self.try_read_exact(&mut crc)?.unwrap();

        Some(Chunk {
            size,
            chunk_type,
            chunk_data,
            crc,
        })
    }

    pub fn try_read_exact(&mut self, buf: &mut [u8]) -> Option<std::io::Result<()>> {
        let result = self.buffer.read_exact(buf);
        if let Err(ref error) = result {
            match error.kind() {
                std::io::ErrorKind::UnexpectedEof => return None,
                _ => return Some(result),
            }
        }
        Some(Ok(()))
    }
}

impl PNG {
    pub fn load_from_path(path: &Path) -> Self {
        let file: File = File::open(path).unwrap();

        PNG {}
    }
}

#[derive(Debug)]
pub struct Chunk {
    size: u32,
    chunk_type: [u8; 4],
    chunk_data: Vec<u8>,
    crc: [u8; 4],
}

// Enum for all the chunk types
#[derive(Debug)]
pub enum PNGChunk {
    IHDR(IHDR),
    IDAT(IDAT),
    IEND(IEND),
    PLTE(PLTE),
    AncillaryChunk(Chunk),
}

impl From<Chunk> for PNGChunk {
    fn from(chunk: Chunk) -> Self {
        match &chunk.chunk_type {
            IHDR::CODE => PNGChunk::IHDR(chunk.into()),
            IDAT::CODE => PNGChunk::IDAT(chunk.into()),
            IEND::CODE => PNGChunk::IEND(chunk.into()),
            PLTE::CODE => PNGChunk::PLTE(chunk.into()),
            _ => PNGChunk::AncillaryChunk(chunk),
        }
    }
}

#[derive(Debug)]
pub struct IHDR {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
}

impl IHDR {
    const CODE: &'static [u8; 4] = &[0x49, 0x48, 0x44, 0x52];
}

impl From<Chunk> for IHDR {
    fn from(chunk: Chunk) -> Self {
        assert_eq!(chunk.size, 13);
        let data = chunk.chunk_data;
        let width_bytes = data[0..4].try_into().expect("Failed to index width bytes");
        let width: u32 = u32::from_be_bytes(width_bytes);
        let height_bytes = data[4..8].try_into().expect("Failed to read height bytes");
        let height: u32 = u32::from_be_bytes(height_bytes);
        let bit_depth: u8 = *data.get(8).expect("Failed to get bit depth");
        let color_type: u8 = *data.get(9).expect("Failed to get color type");
        let compression_method: u8 = *data.get(10).expect("Failed to get compression method");
        let filter_method: u8 = *data.get(11).expect("Failed to get filter method");
        let interlace_method: u8 = *data.get(12).expect("Failed to get interlace method");

        IHDR {
            width,
            height,
            bit_depth,
            color_type,
            compression_method,
            filter_method,
            interlace_method,
        }
    }
}

#[derive(Debug)]
pub struct IDAT {}
impl IDAT {
    const CODE: &'static [u8; 4] = &[0x49, 0x44, 0x41, 0x54];
}

impl From<Chunk> for IDAT {
    fn from(chunk: Chunk) -> Self {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct IEND {}
impl IEND {
    const CODE: &'static [u8; 4] = &[0x49, 0x45, 0x4E, 0x44];
}

impl From<Chunk> for IEND {
    fn from(chunk: Chunk) -> Self {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct PLTE {}
impl PLTE {
    const CODE: &'static [u8; 4] = &[0x50, 0x4C, 0x54, 0x45];
}

impl From<Chunk> for PLTE {
    fn from(chunk: Chunk) -> Self {
        unimplemented!()
    }
}

/// We are ignoring all of these chunks for the first implementation
/// as they are not 'critical chunk's'
#[derive(Debug)]
#[allow(unused, non_camel_case_types)]
struct bkGD {}

#[derive(Debug)]
#[allow(unused, non_camel_case_types)]
struct cHRM {}

#[derive(Debug)]
#[allow(unused, non_camel_case_types)]
struct gAMA {}

#[derive(Debug)]
#[allow(unused, non_camel_case_types)]
struct pHYs {}

#[derive(Debug)]
#[allow(unused, non_camel_case_types)]
struct sBIT {}

#[derive(Debug)]
#[allow(unused, non_camel_case_types)]
struct tEXt {}

#[derive(Debug)]
#[allow(unused, non_camel_case_types)]
struct tIME {}

#[derive(Debug)]
#[allow(unused, non_camel_case_types)]
struct tRNS {}

#[derive(Debug)]
#[allow(unused, non_camel_case_types)]
struct zTXT {}
