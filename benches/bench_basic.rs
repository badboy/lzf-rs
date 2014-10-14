extern crate test;
extern crate lzf;
use test::Bencher;

#[bench]
fn bench_basic_zfs_compression_decompression(b: &mut Bencher) {
    b.iter(|| {
        let lorem = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.";

        let compressed = lzf::compress(lorem.as_bytes()).unwrap();
        let _ = lzf::decompress(compressed.as_slice(), lorem.len());
    })
}
