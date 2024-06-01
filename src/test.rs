use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

// Ignoring error handling for a first pass
struct PNG {}

struct PNGFileBuffer {
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
        let mut signature_buffer: &mut [u8] = &mut [0; 8];
        buff.read_exact(signature_buffer).unwrap();
        assert_eq!(signature_buffer, Self::SIGNATURE);
    }

    fn read_chunk(&mut self) -> PNGChunk {

    }
}

impl PNG {
    pub fn load_from_path(path: &Path) -> Self {
        let file: File = File::open(path).unwrap();

        PNG {}
    }
}

#[derive(Debug)]
pub struct Chunk<'a> {
    size: u32,
    chunk_type: &'a [u8; 4],
    chunk_data: &'a [u8],
    crc: &'a [u8; 4],
}

// Enum for all the chunk types
pub enum PNGChunk {
    IHDR(IHDR),
    IDAT(IDAT),
    IEND(IEND),
    PLTE(PLTE),
    NotImplemented 
}

impl<'a> From<Chunk<'a>> for PNGChunk {
    fn from(chunk: Chunk) -> Self {
        match chunk.chunk_type {
            IHDR::CODE => PNGChunk::IHDR(IHDR {}),
            IDAT::CODE => PNGChunk::IDAT(IDAT {}),
            IEND::CODE => PNGChunk::IEND(IEND {}),
            PLTE::CODE => PNGChunk::PLTE(PLTE {}),
            _ => PNGChunk::NotImplemented
        }
    }
}

struct IHDR {}
impl IHDR {
    const CODE: &'static [u8; 4] = &[0x49, 0x48, 0x44, 0x52];
}

struct IDAT {}
impl IDAT {
    const CODE: &'static [u8; 4] = &[0x49, 0x44, 0x41, 0x54];
}

struct IEND {}
impl IEND {
    const CODE: &'static [u8; 4] = &[0x49, 0x45, 0x4E, 0x44];
}

struct PLTE {}
impl PLTE {
    const CODE: &'static [u8; 4] = &[0x50, 0x4C, 0x54, 0x45];
}

struct bkGD {}
struct cHRM {}
struct gAMA {}
struct pHYs {}
struct sBIT {}
struct tEXt {}
struct tIME {}
struct tRNS {}
struct zTXT {}
