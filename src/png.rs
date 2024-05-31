
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
