# LZF - a very small data compression library

[![crates.io](http://meritbadge.herokuapp.com/lzf)](https://crates.io/crates/lzf)
[![Build Status](https://travis-ci.org/badboy/lzf-rs.svg?branch=master)](https://travis-ci.org/badboy/lzf-rs)

[LibLZF][] is a super small and fast compression library, originally written by Marc Lehmann.
It's written in C and consists of only 4 files.
And this is the rewrite in Rust.

~~Instead of rewriting the whole thing in Rust, I used Rust's [Foreign Function Interface][ffi] and wrote a wrapper.
The whole Rust code is under 50 lines (yes, there is more test code than actual implementation code).
And it is super easy to use, though I'm not happy with the current interface.~~

I sat down and tried to understand the original C code and then rewrote it in (mostly) safe Rust code.
And the best thing: it's still super fast (on some basic benchmarks it's nearly as fast as the original code).
It now consists of roughly 200 lines of code, which is probably around the same as the original implementation.


## Build

```
cargo build --release
```

## Usage

```rust
extern crate lzf;

fn main() {
  let data = "foobar";

  let compressed = lzf::compress(data.as_bytes()).unwrap();

  let decompressed = lzf::decompress(&compressed).unwrap();
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

## Contribute

If you find bugs or want to help otherwise, please [open an issue](https://github.com/badboy/lzf-rs/issues).  
This is my first released library in Rust and I'm still learning. So if there are better ways to do things in Rust, I'm happy to hear about it.

## License

BSD. See [LICENSE](LICENSE).  
liblzf is BSD as well. See [lzf.h](lzf/lzf.h).

[liblzf]: http://software.schmorp.de/pkg/liblzf.html
[ffi]: http://doc.rust-lang.org/guide-ffi.html
