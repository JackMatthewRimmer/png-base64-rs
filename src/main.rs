use png_base64_rs::PNG;
use std::fs;

fn main() {
    let file_contents = fs::read("src/test-image.png").unwrap();
    let _png: PNG = PNG::from_bytes(&file_contents);
}
