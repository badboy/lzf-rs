extern crate lzf;
use lzf::compress;
use lzf::decompress;

fn main() {
    let lorem = "\r\n\r\n\r\n\r\n                ALICE'S ADVENTURES IN WONDERLAND\r\n";

    println!("lorem.len: {}", lorem.len());

    let compressed = compress(lorem.as_bytes()).unwrap();
    println!("l: {}", compressed.len());

    let mut decompressed = vec![0; lorem.len()];
    let len = decompress(&compressed[..], &mut decompressed).unwrap();
    println!("l: {:?}", len);
}
