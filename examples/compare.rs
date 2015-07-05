extern crate lzf;
use lzf::native;
use std::io::{self, Read, Write};
use std::fs::File;

fn main() {
    let mut contents: Vec<u8> = Vec::new();
    let _ = io::stdin().read_to_end(&mut contents).unwrap();
    println!("len:          {}", contents.len());

    let native_compressed = native::compress(&contents[..]).unwrap();
    let compressed        = lzf::compress(&contents[..]).unwrap();
    println!("compressed:   {}", compressed.len());

    assert_eq!(native_compressed.len(), compressed.len());
    assert_eq!(&native_compressed[..], &compressed[..]);

    let native_decompressed = native::decompress(&compressed[..], contents.len()).unwrap();
    let decompressed = lzf::decompress(&compressed[..], contents.len()).unwrap();
    println!("decompressed: {}", decompressed.len());

    assert_eq!(native_decompressed.len(), decompressed.len());
    assert_eq!(&native_decompressed[..], &decompressed[..]);

    let mut f = File::create("decompressed.bin").unwrap();
    f.write_all(&decompressed[..]).unwrap();

    let mut f = File::create("native_decompressed.bin").unwrap();
    f.write_all(&native_decompressed[..]).unwrap();
}
