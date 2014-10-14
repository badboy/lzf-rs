# LZF - a very small data compression library

[LibLZF]() is a super small and fast compression library, originally written by Marc Lehmann.
It's written in C and consists of only 4 files.

Instead of rewriting the whole thing in Rust, I used Rust's [Foreign Function Interface][ffi] and wrote a wrapper.
The whole Rust code is under 50 lines (yes, there is more test code than actual implementation code).
And it is super easy to use, though I'm not happy with the current interface.


## Build

```
cargo build --release
```

## Usage

```rust
use lzf;

fn main() {
  let data = "foobar";

  let compressed = lzf::compress(data.as_bytes()).unwrap();

  let decompressed = lzf::decompress(compressed.as_slice(), data.len()).unwrap();
}

```

## Tests

Run tests with:

```
cargo test
```

Run benchmarks with:

```
cargo bench
```

## License

BSD. See [LICENSE](LICENSE).  
liblzf is BSD as well. See [lzf.h](lzf/lzf.h).

[liblzf]: http://software.schmorp.de/pkg/liblzf.html
[ffi]: http://doc.rust-lang.org/guide-ffi.html
