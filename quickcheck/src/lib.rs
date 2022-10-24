pub mod sys {
    use libc::{c_uint,c_void};
    use lzf::{LzfError, LzfResult};
    use std::io::Error;

    extern {
        fn lzf_compress(in_data: *const c_void, in_len: c_uint,
                        out_data: *const c_void, out_len: c_uint) -> c_uint;
        fn lzf_decompress(in_data: *const c_void, in_len: c_uint,
                          out_data: *const c_void, out_len: c_uint) -> c_uint;
    }

    pub fn compress(data: &[u8]) -> LzfResult<Vec<u8>> {
        let data_len = data.len();
        let mut out : Vec<u8> = Vec::with_capacity(data_len);

        let result = unsafe { lzf_compress(data.as_ptr() as *const c_void, data_len as c_uint,
        out.as_ptr() as *const c_void, data_len as c_uint) };

        match result {
            0 => Err(LzfError::NoCompressionPossible),
            _ => {
                unsafe { out.set_len(result as usize) };
                Ok(out)
            }
        }
    }

    pub fn decompress(data: &[u8], out_len: usize) -> LzfResult<Vec<u8>> {
        let mut out : Vec<u8> = Vec::with_capacity(out_len);

        if data.len() == 0 {
            return Err(LzfError::DataCorrupted);
        }

        let result = unsafe { lzf_decompress(data.as_ptr() as *const c_void, data.len() as c_uint,
        out.as_ptr() as *const c_void, out_len as c_uint) };
        match result {
            0 => {
                match Error::last_os_error().raw_os_error() {
                    Some(7)  => Err(LzfError::BufferTooSmall),
                    Some(22) => Err(LzfError::DataCorrupted),
                    Some(e)  => Err(LzfError::UnknownError(e)),
                    None     => Err(LzfError::UnknownError(0)),
                }
            },
            _ => {
                unsafe { out.set_len(result as usize) };
                Ok(out)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::sys;
    use lzf::LzfError;

    #[test]
    fn test_compress_skips_short() {
        match sys::compress("foo".as_bytes()) {
            Ok(_) => panic!("Compression did _something_, with is wrong for 'foo'"),
            Err(err) => assert_eq!(LzfError::NoCompressionPossible, err)
        }
    }

    #[test]
    fn test_compress_lorem() {
        let lorem = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.";

        match sys::compress(lorem.as_bytes()) {
            Ok(compressed) => {
                assert_eq!(272, compressed.len())
            }
            Err(err) => panic!("Compression failed with error {:?}", err)
        }
    }

    #[test]
    fn test_compress_decompress_lorem_round() {
        let lorem = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.";

        let compressed = match sys::compress(lorem.as_bytes()) {
            Ok(c) => c,
            Err(err) => panic!("Compression failed with error {:?}", err)
        };

        match sys::decompress(&compressed, lorem.len()) {
            Ok(decompressed) => {
                assert_eq!(lorem.len(), decompressed.len());
                assert_eq!(lorem.as_bytes(), &decompressed[..]);
            },
            Err(err) => panic!("Decompression failed with error {:?}", err)
        };
    }

    #[test]
    fn test_decompress_fails_with_short_buffer() {
        let lorem = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.";

        let compressed = match sys::compress(lorem.as_bytes()) {
            Ok(c) => c,
            Err(err) => panic!("Compression failed with error {:?}", err)
        };

        match sys::decompress(&compressed, 10) {
            Ok(_) => panic!("Decompression worked. That should not happen"),
            Err(err) => assert_eq!(LzfError::BufferTooSmall, err)
        }
    }

    #[test]
    fn test_decompress_fails_for_corrupted_data() {
        let lorem = "Lorem ipsum dolor sit amet";

        match sys::decompress(lorem.as_bytes(), lorem.len()) {
            Ok(_) => panic!("Decompression worked. That should not happen"),
            Err(err) => assert_eq!(LzfError::DataCorrupted, err)
        }
    }

    #[test]
    fn test_empty() {
        assert_eq!(LzfError::DataCorrupted, sys::decompress(&[], 10).unwrap_err());
    }
}

#[cfg(test)]
mod quickcheck_test {
    use super::sys;
    use quickcheck::{quickcheck, TestResult};
    use lzf::{self, LzfError};

    fn roundtrip_native(data: Vec<u8>) -> TestResult {
        let compr = match sys::compress(&data) {
            Ok(compr) => compr,
            Err(LzfError::NoCompressionPossible) => return TestResult::discard(),
            Err(LzfError::DataCorrupted) => return TestResult::discard(),
            e @ _ => panic!("{:?}", e),
        };
        let decompr = sys::decompress(&compr, data.len()).unwrap();
        TestResult::from_bool(data == decompr)
    }

    #[test]
    fn qc_native_roundtrip() {
        quickcheck(roundtrip_native as fn(_) -> _);
    }

    fn compare_compress(data: Vec<u8>) -> TestResult {
        let rust_compr = lzf::compress(&data);
        let native_compr = sys::compress(&data);
        TestResult::from_bool(rust_compr == native_compr)
    }

    #[test]
    fn qc_native_matches_rust() {
        quickcheck(compare_compress as fn(_) -> _);
    }

    fn compare_decompress(data: Vec<u8>) -> TestResult {
        let rust_decompr = lzf::decompress(&data, data.len()*2);
        let native_decompr = sys::decompress(&data, data.len()*2);
        TestResult::from_bool(rust_decompr == native_decompr)
    }

    #[test]
    fn qc_native_decompress_matches_rust() {
        quickcheck(compare_decompress as fn(_) -> _);
    }

    fn native_compress_rust_decompress(data: Vec<u8>) -> TestResult {
        let compr = match sys::compress(&data) {
            Ok(compr) => compr,
            Err(LzfError::NoCompressionPossible) => return TestResult::discard(),
            Err(LzfError::DataCorrupted) => return TestResult::discard(),
            e @ _ => panic!("{:?}", e),
        };

        let decompr = lzf::decompress(&compr, data.len()).unwrap();
        TestResult::from_bool(data == decompr)
    }

    fn rust_compress_native_decompress(data: Vec<u8>) -> TestResult {
        let compr = match lzf::compress(&data) {
            Ok(compr) => compr,
            Err(LzfError::NoCompressionPossible) => return TestResult::discard(),
            Err(LzfError::DataCorrupted) => return TestResult::discard(),
            e @ _ => panic!("{:?}", e),
        };

        let decompr = sys::decompress(&compr, data.len()).unwrap();
        TestResult::from_bool(data == decompr)
    }

    #[test]
    fn qc_native_compress_rust_decompress() {
        quickcheck(native_compress_rust_decompress as fn(_) -> _);
    }

    #[test]
    fn qc_rust_compress_native_decompress() {
        quickcheck(rust_compress_native_decompress as fn(_) -> _);
    }
}
