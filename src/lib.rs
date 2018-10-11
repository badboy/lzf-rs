//! lzf is a very small data compression library.
//!
//! Originally written as [LibLZF](http://software.schmorp.de/pkg/liblzf.html)
//! by Marc Lehmann in portable C.
//!
//! This Rust library is a rewrite of the original C code
//! and fully compatible with compressed data from the C code (and vice versa).
//!
//! # Basic Operation
//!
//! ```rust
//! # use lzf;
//! let data = "aaaaaaaaa";
//!
//! let compressed = lzf::compress(data.as_bytes()).unwrap();
//!
//! let decompressed = lzf::decompress(&compressed).unwrap();
//! ```
#![deny(missing_docs)]

#[cfg(test)]
extern crate quickcheck;

use std::fmt;

mod compress;
mod decompress;
pub use compress::compress;
pub use decompress::decompress;

/// Errors that can occur during Compression or Decompression.
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum LzfError {
    /// The provided buffer is too small to handle the uncompressed data
    BufferTooSmall,
    /// The given compressed data is corrupted
    DataCorrupted,
    /// The given data can't be compressed
    NoCompressionPossible,
    /// An unknown error occured
    UnknownError(i32),
}

impl fmt::Display for LzfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LzfError::BufferTooSmall => {
                write!(f,
                       "the given buffer is too small to handle the uncompressed data")
            }
            LzfError::DataCorrupted => {
                write!(f, "the given data is corrupted")
            }
            LzfError::NoCompressionPossible => {
                write!(f, "the input data cannot be compressed")
            }
            LzfError::UnknownError(err) => {
                write!(f, "unknown error, code {}", err)
            }
        }
    }
}

/// A Result providing the underlying data or a compression/decompression error
pub type LzfResult<T> = Result<T, LzfError>;

#[test]
fn test_compress_skips_short() {
    match compress("foo".as_bytes()) {
        Ok(_) => panic!("Compression did _something_, which is wrong for 'foo'"),
        Err(err) => assert_eq!(LzfError::NoCompressionPossible, err),
    }
}

#[test]
fn test_compress_lorem() {
    let lorem = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod \
                 tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At \
                 vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, \
                 no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit \
                 amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut \
                 labore et dolore magna aliquyam erat, sed diam voluptua.";

    match compress(lorem.as_bytes()) {
        Ok(compressed) => {
            assert_eq!(272, compressed.len())
        }
        Err(err) => panic!("Compression failed with error {:?}", err),
    }
}

#[test]
fn test_compress_decompress_lorem_round() {
    let lorem = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod \
                 tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At \
                 vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, \
                 no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit \
                 amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut \
                 labore et dolore magna aliquyam erat, sed diam voluptua.";

    let compressed = match compress(lorem.as_bytes()) {
        Ok(c) => c,
        Err(err) => panic!("Compression failed with error {:?}", err),
    };

    match decompress(&compressed) {
        Ok(decompressed) => {
            assert_eq!(lorem.len(), decompressed.len());
            assert_eq!(lorem.as_bytes(), &decompressed[..]);
        }
        Err(err) => panic!("Decompression failed with error {:?}", err),
    };
}

#[cfg(test)]
mod quickcheck_test {
    use super::*;
    use quickcheck::{quickcheck, TestResult};

    fn compress_decompress_round(data: Vec<u8>) -> TestResult {
        let compr = match compress(&data) {
            Ok(compr) => compr,
            Err(LzfError::NoCompressionPossible) => return TestResult::discard(),
            Err(LzfError::DataCorrupted) => return TestResult::discard(),
            e @ _ => panic!(e),
        };
        let decompr = decompress(&compr).unwrap();
        TestResult::from_bool(data == decompr)
    }

    #[test]
    fn qc_roundtrip() {
        quickcheck(compress_decompress_round as fn(_) -> _);
    }
}
