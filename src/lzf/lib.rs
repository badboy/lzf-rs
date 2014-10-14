//! lzf-rs is a small wrapper around [LibLZF](http://software.schmorp.de/pkg/liblzf.html),
//! a very small data compression library.
//!
//! The compression algorithm is very, very fast, yet still written in portable C.
//!
//! This Rust library is a wrapper around the library from Marc Lehmann.
//!
//! # Basic Operation
//!
//! ```rust,no_run
//! # use lzf;
//! let data = "foobar";
//!
//! let compressed = lzf::compress(data.as_bytes()).unwrap();
//!
//! let decompressed = lzf::decompress(compressed.as_slice(), data.len()).unwrap();
//! ```

#![crate_name = "lzf"]
#![crate_type = "lib"]
#![license = "BSD"]
#![comment = "Bindings for LibLZF"]

#![experimental]

extern crate libc;

pub use lzf::{compress,decompress,LzfError,LzfResult};
mod lzf;
