use super::{LzfResult, LzfError};

/// Decompress the given data, if possible.
/// An error will be returned if decompression fails.
///
/// The given output buffer will be filled with the data and the length of the decompressed data
/// will be returned.
/// The slice should only be handled up to this length.
/// If the output buffer is not large enough to hold the decompressed data,
/// a `BufferTooSmall` error is returned.
/// Otherwise the number of decompressed bytes
/// (i.e. the original length of the data) is returned.
///
/// If an error in the compressed data is detected, `DataCorrupted` is returned.
///
/// Example:
///
/// ```rust,no_run
/// let data = "[your-compressed-data]";
/// let mut decompressed = vec![0, 10];
/// let bytes = lzf::decompress(data.as_bytes(), &mut decompressed).unwrap();
/// ```
pub fn decompress(data: &[u8], output: &mut [u8]) -> LzfResult<usize> {
    let mut current_offset = 0;

    let in_len = data.len();
    if in_len == 0 {
        return Err(LzfError::DataCorrupted);
    }

    // We have sanity checks to not exceed this capacity.
    let out_len_should = output.len();
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
            output[out_len..(out_len+ctrl)].copy_from_slice(&data[current_offset..(current_offset+ctrl)]);

            current_offset += ctrl;
            out_len += ctrl;
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
    Ok(out_len)
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

    let mut decompressed = vec![0; lorem.len()];
    let len = decompress(&compressed[..], &mut decompressed).unwrap();
    assert_eq!(lorem.as_bytes(), &decompressed[..len]);

    let mut decompressed = vec![0; 1000];
    let len = decompress(&compressed[..], &mut decompressed).unwrap();
    assert_eq!(lorem.len(), len);
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

    let mut buffer = [0; 10];
    match decompress(&compressed, &mut buffer) {
        Ok(_) => panic!("Decompression worked. That should not happen"),
        Err(err) => assert_eq!(LzfError::BufferTooSmall, err),
    }
}

#[test]
fn test_decompress_fails_for_corrupted_data() {
    let lorem = "Lorem ipsum dolor sit amet";

    let mut buffer= vec![0; lorem.len()];
    match decompress(lorem.as_bytes(), &mut buffer) {
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

    let mut decompressed = vec![0; alice.len()];
    match decompress(&compressed, &mut decompressed) {
        Ok(len) => {
            assert_eq!(alice.len(), len);
            assert_eq!(alice.as_bytes(), &decompressed[..len]);
        }
        Err(err) => panic!("Decompression failed with error {:?}", err),
    }
}

#[test]
fn easily_compressible() {
    // RDB regression
    let data = vec![1, 97, 97, 224, 187, 0, 1, 97, 97];

    let mut text = [0; 200];
    let len = decompress(&data, &mut text).unwrap();
    assert_eq!(200, len);
    assert_eq!(97, text[0]);
    assert_eq!(97, text[199]);
}

#[test]
fn test_empty() {
    let mut buf = [0; 10];
    assert_eq!(LzfError::DataCorrupted, decompress(&[], &mut buf).unwrap_err());
}
