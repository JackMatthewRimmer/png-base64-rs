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
            crc
        })
    }

    pub fn try_read_exact(&mut self,  buf: &mut [u8]) -> Option<std::io::Result<()>> {
        let result = self.buffer.read_exact(buf);
        if let Err(ref error) = result {
            match error.kind() {
                std::io::ErrorKind::UnexpectedEof => return None,
                _ => return Some(result)

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
    NotImplemented 
}

impl From<Chunk> for PNGChunk {
    fn from(chunk: Chunk) -> Self {
        match &chunk.chunk_type {
            IHDR::CODE => PNGChunk::IHDR(IHDR {}),
            IDAT::CODE => PNGChunk::IDAT(IDAT {}),
            IEND::CODE => PNGChunk::IEND(IEND {}),
            PLTE::CODE => PNGChunk::PLTE(PLTE {}),
            _ => PNGChunk::NotImplemented
        }
    }
}

#[derive(Debug)]
struct IHDR {}
impl IHDR {
    const CODE: &'static [u8; 4] = &[0x49, 0x48, 0x44, 0x52];
}

#[derive(Debug)]
struct IDAT {}
impl IDAT {
    const CODE: &'static [u8; 4] = &[0x49, 0x44, 0x41, 0x54];
}

#[derive(Debug)]
struct IEND {}
impl IEND {
    const CODE: &'static [u8; 4] = &[0x49, 0x45, 0x4E, 0x44];
}

#[derive(Debug)]
struct PLTE {}
impl PLTE {
    const CODE: &'static [u8; 4] = &[0x50, 0x4C, 0x54, 0x45];
}

#[derive(Debug)]
struct bkGD {}

#[derive(Debug)]
struct cHRM {}

#[derive(Debug)]
struct gAMA {}

#[derive(Debug)]
struct pHYs {}

#[derive(Debug)]
struct sBIT {}

#[derive(Debug)]
struct tEXt {}

#[derive(Debug)]
struct tIME {}

#[derive(Debug)]
struct tRNS {}

#[derive(Debug)]
struct zTXT {}
