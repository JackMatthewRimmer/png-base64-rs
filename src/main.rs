use png_base64_rs::PNGFileBuffer;
use std::fs::File;

fn main() {
    let file = File::open("src/test-image.png").unwrap();
    let mut buffer = PNGFileBuffer::new(file); 

    while let Some(chunk) = buffer.read_chunk() {
        dbg!(chunk);
    }
}
