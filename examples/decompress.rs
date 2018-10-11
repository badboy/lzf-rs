extern crate lzf;
use std::io::{self, Read, Write};

fn main() {
    let mut contents: Vec<u8> = Vec::new();
    let _ = io::stdin().read_to_end(&mut contents).unwrap();

    let decompressed = lzf::decompress(&contents[..], contents.len()*2).unwrap();
    let _ = io::stdout().write_all(&decompressed[..]).unwrap();
}
