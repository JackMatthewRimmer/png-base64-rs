use png_base64_rs::{PNGFileBuffer, PNGChunk};
use std::fs::File;

fn main() {
    let file = File::open("src/test-image.png").unwrap();
    let mut buffer = PNGFileBuffer::new(file); 

    while let Some(chunk) = buffer.read_chunk() {
        let png_chunk: PNGChunk = chunk.into();
        dbg!(png_chunk);
    }
}
