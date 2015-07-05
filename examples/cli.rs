extern crate lzf;
use lzf::native;
use std::io::{self, Read, Write};

fn main() {
    let mut contents: Vec<u8> = Vec::new();
    let _ = io::stdin().read_to_end(&mut contents).unwrap();

    let compressed = native::compress(&contents[..]).unwrap();
    let _ = io::stdout().write_all(&compressed[..]).unwrap();
}
