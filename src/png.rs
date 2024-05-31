
pub struct PNG {}

impl PNG {
    const SIGNATURE: &'static [u8] = &[137, 80, 78, 71, 13, 10, 26, 10];

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let chunk_bytes: &[u8] = Self::read_and_check_file_signature(bytes);
        PNG {}
    }

    fn read_and_check_file_signature<'a>(bytes: &'a [u8]) -> &'a [u8] {
        let signature: &[u8] = &bytes[0..8];
        assert_eq!(signature, Self::SIGNATURE);
        &bytes[8..]
    }
}


/// Idea here is read in a raw chunk and then parse it based on the chunk_type
pub struct Chunk<'a> {
    size: &'a [u8; 4],
    chunk_type: &'a [u8; 4],
    chunk_data: &'a [u8],
    crc: &'a [u8; 4]
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


