pub struct PNG {}

impl PNG {
    const SIGNATURE: &'static [u8] = &[137, 80, 78, 71, 13, 10, 26, 10];

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut chunk_bytes: &[u8] = Self::read_and_check_file_signature(bytes);

        while !chunk_bytes.is_empty() {
            let (chunk, rem) = Self::read_chunk(chunk_bytes);
            chunk_bytes = rem;

            println!("Chunk type {}", chunk.dbg_type());
            println!("This is a chunk {:?}", chunk);
        }

        PNG {}
    }

    fn read_and_check_file_signature<'a>(bytes: &'a [u8]) -> &'a [u8] {
        let signature: &[u8] = &bytes[0..8];
        assert_eq!(signature, Self::SIGNATURE);
        &bytes[8..]
    }

    // Function returns a chunk and the remaining bytes
    fn read_chunk<'a>(bytes: &'a [u8]) -> (Chunk<'a>, &'a [u8]) {
        let size_bytes: [u8; 4] = bytes
            .get(0..4)
            .and_then(|size_bytes| size_bytes.try_into().ok())
            .expect("Failed to read chunk size");

        let size_32: u32 = u32::from_be_bytes(size_bytes);
        let end_of_data_index: usize = (size_32 + 8) as usize;

        let chunk_type: &[u8; 4] = bytes
            .get(4..8)
            .and_then(|size_bytes| size_bytes.try_into().ok())
            .expect("Failed to read chunk type");

        let chunk_data: &[u8] = bytes
            .get(8..end_of_data_index)
            .expect("Failed to read to chunk data");

        let crc: &[u8; 4] = bytes
            .get(end_of_data_index..end_of_data_index + 4)
            .and_then(|crc_bytes| crc_bytes.try_into().ok())
            .expect("Failed to read crc");

        let chunk: Chunk = Chunk {
            size: size_32,
            chunk_type,
            chunk_data,
            crc,
        };

        let remaining_bytes = bytes
            .get(end_of_data_index + 4..)
            .expect("Failed to get remaining bytes");

        (chunk, remaining_bytes)
    }
}

/// Idea here is read in a raw chunk and then parse it based on the chunk_type
#[derive(Debug)]
pub struct Chunk<'a> {
    size: u32,
    chunk_type: &'a [u8; 4],
    chunk_data: &'a [u8],
    crc: &'a [u8; 4],
}

impl<'a> Chunk<'a> {
    const IHDR_CODE: &'static [u8; 4] = &[0x49, 0x48, 0x44, 0x52];
    const IDAT_CODE: &'static [u8; 4] = &[0x49, 0x44, 0x41, 0x54];
    const PLTE_CODE: &'static [u8; 4] = &[0x50, 0x4C, 0x54, 0x45];
    const IEND_CODE: &'static [u8; 4] = &[0x49, 0x45, 0x4E, 0x44];

    pub fn dbg_type(&'a self) -> &str {
        match self.chunk_type {
            Self::IHDR_CODE => "IHDR",
            Self::IDAT_CODE => "IDAT",
            Self::IEND_CODE => "IEND",
            Self::PLTE_CODE => "PLTE",
            _ => "Ancillary",
        }
    }
}

/// Been getting them from here [https://www.w3.org/TR/PNG-Chunks.html#:~:text=A%20valid%20PNG%20image%20must,chunks%2C%20and%20an%20IEND%20chunk.]
/// this is all the structs for each chunk type possible
struct IHDR {}
struct IDAT {}
struct IEND {}
struct PLTE {}
struct bkGD {}
struct cHRM {}
struct gAMA {}
struct pHYs {}
struct sBIT {}
struct tEXt {}
struct tIME {}
struct tRNS {}
struct zTXT {}

// I think these can be ignored so maybe no point ?
struct AnicillaryChunk {}
