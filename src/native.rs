use super::{LzfResult,LzfError};
use std::ptr;
use std::mem;

pub fn decompress(data: &[u8], out_len_should: usize) -> LzfResult<Vec<u8>> {
    let mut current_offset = 0;

    // We have sanity checks to not exceed this capacity.
    let mut output = Vec::with_capacity(out_len_should);
    unsafe { output.set_len(out_len_should) };
    let mut out_len : usize = 0;

    let in_len = data.len();

    while current_offset < in_len {
        let mut ctrl = unsafe{*data.get_unchecked(current_offset)} as usize;
        current_offset += 1;

        if ctrl < (1<<5) {
            ctrl += 1;

            if out_len + ctrl > out_len_should {
                return Err(LzfError::BufferTooSmall);
            }

            if current_offset+ctrl > in_len {
                return Err(LzfError::DataCorrupted);
            }

            // We can simply memcpy everything from the input to the output
            unsafe {
                let (src, _) : (*const u8, usize) = mem::transmute(&data[..]);
                let src = src.offset(current_offset as isize);
                let (dst, _) : (*mut u8, usize) = mem::transmute(&output[..]);
                let dst = dst.offset((out_len) as isize);
                ptr::copy_nonoverlapping(src, dst, ctrl);

                current_offset += ctrl;
                out_len += ctrl;
            }
        } else {
            let mut len = ctrl >> 5;

            let mut ref_offset = (((ctrl & 0x1f) << 8) + 1) as i32;

            if len == 7 {
                len += unsafe{*data.get_unchecked(current_offset)} as usize;
                current_offset += 1;

                if current_offset >= in_len {
                    return Err(LzfError::DataCorrupted);
                }
            }

            ref_offset += unsafe{*data.get_unchecked(current_offset)} as i32;
            current_offset += 1;

            if current_offset + len + 2 > out_len_should {
                return Err(LzfError::BufferTooSmall);
            }

            let mut ref_pos = (out_len as i32) - ref_offset;
            if ref_pos < 0 {
                return Err(LzfError::DataCorrupted);
            }

            let c = unsafe{*output.get_unchecked(ref_pos as usize)};
            output[out_len] = c;
            out_len += 1;
            ref_pos += 1;

            let c = unsafe{*output.get_unchecked(ref_pos as usize)};
            output[out_len] = c;
            out_len += 1;
            ref_pos += 1;

            // We can safely use copy_nonoverlapping here,
            // as we know that we only copy data from before our current insertion point.
            unsafe {
                let (src, _) : (*const u8, usize) = mem::transmute(&output[..]);
                let src = src.offset(ref_pos as isize);
                let (dst, _) : (*mut u8, usize) = mem::transmute(&output[..]);
                let dst = dst.offset((out_len) as isize);

                ptr::copy_nonoverlapping(src, dst, len);
                out_len += len;
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

    let lorem = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.";

    let compressed = match compress(lorem.as_bytes()) {
        Ok(c) => c,
        Err(err) => panic!("Compression failed with error {:?}", err)
    };

    match decompress(&compressed, 10) {
        Ok(_) => panic!("Decompression worked. That should not happen"),
        Err(err) => assert_eq!(LzfError::BufferTooSmall, err)
    }
}

#[test]
fn test_decompress_fails_for_corrupted_data() {
    let lorem = "Lorem ipsum dolor sit amet";

    match decompress(lorem.as_bytes(), lorem.len()) {
        Ok(_) => panic!("Decompression worked. That should not happen"),
        Err(err) => assert_eq!(LzfError::DataCorrupted, err)
    }
}
