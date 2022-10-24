#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn bench_basic_lzf_compression_decompression(b: &mut Bencher) {
    b.iter(|| {
        let lorem = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.";

        let compressed = lzf::compress(lorem.as_bytes()).unwrap();
        let _ = lzf::decompress(&compressed, lorem.len());
    })
}

#[bench]
fn bench_basic_lzf_decompression(b: &mut Bencher) {
    b.iter(|| {
        let lorem = [
            31, 76, 111, 114, 101, 109, 32, 105, 112, 115, 117, 109, 32, 100, 111, 108, 111, 114,
            32, 115, 105, 116, 32, 97, 109, 101, 116, 44, 32, 99, 111, 110, 115, 4, 101, 116, 101,
            116, 117, 32, 20, 1, 97, 100, 32, 35, 31, 99, 105, 110, 103, 32, 101, 108, 105, 116,
            114, 44, 32, 115, 101, 100, 32, 100, 105, 97, 109, 32, 110, 111, 110, 117, 109, 121,
            32, 101, 105, 114, 109, 6, 111, 100, 32, 116, 101, 109, 112, 32, 68, 14, 105, 110, 118,
            105, 100, 117, 110, 116, 32, 117, 116, 32, 108, 97, 98, 32, 100, 2, 32, 101, 116, 128,
            96, 13, 101, 32, 109, 97, 103, 110, 97, 32, 97, 108, 105, 113, 117, 121, 32, 64, 2,
            101, 114, 97, 32, 108, 224, 0, 79, 20, 118, 111, 108, 117, 112, 116, 117, 97, 46, 32,
            65, 116, 32, 118, 101, 114, 111, 32, 101, 111, 115, 64, 61, 4, 97, 99, 99, 117, 115,
            64, 47, 9, 116, 32, 106, 117, 115, 116, 111, 32, 100, 117, 32, 3, 64, 179, 0, 101, 96,
            31, 10, 101, 97, 32, 114, 101, 98, 117, 109, 46, 32, 83, 32, 180, 1, 32, 99, 32, 167,
            16, 97, 32, 107, 97, 115, 100, 32, 103, 117, 98, 101, 114, 103, 114, 101, 110, 44, 32,
            173, 32, 105, 7, 97, 32, 116, 97, 107, 105, 109, 97, 32, 31, 5, 115, 97, 110, 99, 116,
            117, 32, 63, 3, 115, 116, 32, 76, 32, 73, 225, 13, 11, 0, 46, 224, 18, 27, 225, 118,
            39, 1, 97, 46,
        ];

        let _ = lzf::decompress(&lorem, 451);
    })
}

#[bench]
fn bench_basic_lzf_compression(b: &mut Bencher) {
    let lorem = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.";
    b.iter(|| {
        let _ = lzf::compress(lorem.as_bytes()).unwrap();
    })
}
