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
//! ```rust,no_run
//! # use lzf;
//! let data = "foobar";
//!
//! let compressed = lzf::compress(data.as_bytes()).unwrap();
//!
//! let decompressed = lzf::decompress(&compressed, data.len()).unwrap();
//! ```

use std::fmt;

mod compress;
mod decompress;
pub use compress::compress;
pub use decompress::decompress;

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum LzfError {
    BufferTooSmall,
    DataCorrupted,
    NoCompressionPossible,
    UnknownError(i32)
}

impl fmt::Display for LzfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LzfError::BufferTooSmall => {
                write!(f, "the given buffer is too small to handle the uncompressed data")
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

pub type LzfResult<T> = Result<T, LzfError>;

#[test]
fn test_compress_skips_short() {
    match compress("foo".as_bytes()) {
        Ok(_) => panic!("Compression did _something_, which is wrong for 'foo'"),
        Err(err) => assert_eq!(LzfError::NoCompressionPossible, err)
    }
}

#[test]
fn test_compress_lorem() {
    let lorem = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.";

    match compress(lorem.as_bytes()) {
        Ok(compressed) => {
            assert_eq!(272, compressed.len())
        }
        Err(err) => panic!("Compression failed with error {:?}", err)
    }
}

#[test]
fn test_compress_decompress_lorem_round() {
    let lorem = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.";

    let compressed = match compress(lorem.as_bytes()) {
        Ok(c) => c,
        Err(err) => panic!("Compression failed with error {:?}", err)
    };

    match decompress(&compressed, lorem.len()) {
        Ok(decompressed) => {
            assert_eq!(lorem.len(), decompressed.len());
            assert_eq!(lorem.as_bytes(), &decompressed[..]);
        },
        Err(err) => panic!("Decompression failed with error {:?}", err)
    };
}
