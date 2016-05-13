use super::{LzfResult, LzfError};
use std::ptr;
use std::mem;

/// Decompress the given data, if possible.
/// An error will be returned if decompression fails.
///
/// The length of the output buffer can be specified.
/// If the output buffer is not large enough to hold the decompressed data,
/// BufferTooSmall is returned.
/// Otherwise the number of decompressed bytes
/// (i.e. the original length of the data) is returned.
///
/// If an error in the compressed data is detected, DataCorrupted is returned.
///
/// Example:
///
/// ```rust,no_run
/// let data = "[your-compressed-data]";
/// let decompressed = lzf::decompress(data.as_bytes(), 10);
/// ```
pub fn decompress(data: &[u8], out_len_should: usize) -> LzfResult<Vec<u8>> {
    let mut current_offset = 0;

    let in_len = data.len();
    if in_len == 0 {
        return Err(LzfError::DataCorrupted);
    }

    // We have sanity checks to not exceed this capacity.
    let mut output = Vec::with_capacity(out_len_should);
    unsafe { output.set_len(out_len_should) };
    let mut out_len: usize = 0;


    while current_offset < in_len {
        let mut ctrl = data[current_offset] as usize;
        current_offset += 1;

        if ctrl < (1 << 5) {
            ctrl += 1;

            if out_len + ctrl > out_len_should {
                return Err(LzfError::BufferTooSmall);
            }

            if current_offset + ctrl > in_len {
                return Err(LzfError::DataCorrupted);
            }

            // We can simply memcpy everything from the input to the output
            unsafe {
                let (src, _): (*const u8, usize) = mem::transmute(&data[..]);
                let src = src.offset(current_offset as isize);
                let (dst, _): (*mut u8, usize) = mem::transmute(&output[..]);
                let dst = dst.offset((out_len) as isize);
                ptr::copy_nonoverlapping(src, dst, ctrl);

                current_offset += ctrl;
                out_len += ctrl;
            }
        } else {
            let mut len = ctrl >> 5;

            let mut ref_offset = (((ctrl & 0x1f) << 8) + 1) as i32;

            if current_offset >= in_len {
                return Err(LzfError::DataCorrupted);
            }

            if len == 7 {
                len += data[current_offset] as usize;
                current_offset += 1;

                if current_offset >= in_len {
                    return Err(LzfError::DataCorrupted);
                }
            }

            ref_offset += data[current_offset] as i32;
            current_offset += 1;

            if out_len + len + 2 > out_len_should {
                return Err(LzfError::BufferTooSmall);
            }

            let mut ref_pos = (out_len as i32) - ref_offset;
            if ref_pos < 0 {
                return Err(LzfError::DataCorrupted);
            }

            let c = output[ref_pos as usize];
            output[out_len] = c;
            out_len += 1;
            ref_pos += 1;

            let c = output[ref_pos as usize];
            output[out_len] = c;
            out_len += 1;
            ref_pos += 1;

            while len > 0 {
                let c = output[ref_pos as usize];
                output[out_len] = c;
                out_len += 1;
                ref_pos += 1;
                len -= 1;
            }
        }
    }

    // Set the real length now, user might have passed a bigger buffer in the first place.
    unsafe { output.set_len(out_len) };

    Ok(output)
}

#[test]
fn test_decompress_lorem() {
    use super::compress;

    let lorem = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod \
                 tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At \
                 vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, \
                 no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit \
                 amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut \
                 labore et dolore magna aliquyam erat, sed diam voluptua.";

    let compressed = compress(lorem.as_bytes()).unwrap();

    let decompressed = decompress(&compressed[..], lorem.len()).unwrap();
    assert_eq!(lorem.as_bytes(), &decompressed[..]);

    let decompressed = decompress(&compressed[..], 1000).unwrap();
    assert_eq!(lorem.len(), decompressed.len());
}

#[test]
fn test_decompress_fails_with_short_buffer() {
    use super::compress;

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

    match decompress(&compressed, 10) {
        Ok(_) => panic!("Decompression worked. That should not happen"),
        Err(err) => assert_eq!(LzfError::BufferTooSmall, err),
    }
}

#[test]
fn test_decompress_fails_for_corrupted_data() {
    let lorem = "Lorem ipsum dolor sit amet";

    match decompress(lorem.as_bytes(), lorem.len()) {
        Ok(_) => panic!("Decompression worked. That should not happen"),
        Err(err) => assert_eq!(LzfError::DataCorrupted, err),
    }
}

#[test]
fn test_alice_wonderland() {
    use super::compress;

    let alice = "\r\n\r\n\r\n\r\n                ALICE'S ADVENTURES IN WONDERLAND\r\n";

    let compressed = match compress(alice.as_bytes()) {
        Ok(c) => c,
        Err(err) => panic!("Compression failed with error {:?}", err),
    };

    match decompress(&compressed, alice.len()) {
        Ok(decompressed) => {
            assert_eq!(alice.len(), decompressed.len());
            assert_eq!(alice.as_bytes(), &decompressed[..]);
        }
        Err(err) => panic!("Decompression failed with error {:?}", err),
    }
}

#[test]
fn easily_compressible() {
    // RDB regression
    let data = vec![1, 97, 97, 224, 187, 0, 1, 97, 97];
    let real_length = 200;

    let text = decompress(&data, real_length).unwrap();
    assert_eq!(200, text.len());
    assert_eq!(97, text[0]);
    assert_eq!(97, text[199]);
}

#[test]
fn test_empty() {
    assert_eq!(LzfError::DataCorrupted, decompress(&[], 10).unwrap_err());
}
