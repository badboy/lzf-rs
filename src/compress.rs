use super::{LzfError, LzfResult};
use std::{cmp, mem};

const HLOG: usize = 16;
const HSIZE: u32 = 1 << HLOG;
const MAX_OFF: usize = 1 << 13;
const MAX_REF: usize = (1 << 8) + (1 << 3);
const MAX_LIT: i32 = 1 << 5;

fn first(p: &[u8], off: usize) -> u32 {
    ((p[off] as u32) << 8) | p[off + 1] as u32
}

fn next(v: u32, p: &[u8], off: usize) -> u32 {
    (v << 8) | p[off + 2] as u32
}

fn idx(h: u32) -> usize {
    let h = h as u64;
    (
        // 8 = 3*8-HLOG, but HLOG is constant at 16
        (h.wrapping_shr(8).wrapping_sub(h * 5)) & (HSIZE - 1) as u64
    ) as usize
}

fn not(i: i32) -> i32 {
    if i == 0 {
        1
    } else {
        0
    }
}

/// Compress the given data, if possible.
/// The return value will be set to the error if compression fails.
///
/// The buffer is always set to the same size as the input buffer.
/// If that is not enough to hold the lzf-compressed data,
/// an error will be returned.
///
/// Example:
///
/// ```rust
/// let data = "aaaaaaaaa";
/// let compressed = lzf::compress(data.as_bytes()).unwrap();
/// ```
pub fn compress(data: &[u8]) -> LzfResult<Vec<u8>> {
    let in_len = data.len();
    let out_buf_len = in_len;
    let mut out = Vec::with_capacity(out_buf_len);
    unsafe { out.set_len(out_buf_len) };

    let mut out_len: i32 = 1; /* start run by default */

    /* This goes against all of Rust's statically verifiable guarantees,
     * but for the below use-case accessing uninitialized memory is ok,
     * as we have other checks to make sure the read memory is not used.
     *
     * The otherwise happening memset slows down the code by a factor of 20-30
     */
    let mut htab: [usize; 1 << HLOG] = unsafe { mem::uninitialized() };

    let mut current_offset = 0;

    if in_len < 2 {
        return Err(LzfError::NoCompressionPossible);
    }

    let mut lit: i32 = 0;

    let mut hval: u32;
    let mut ref_offset;

    hval = first(data, current_offset);

    while current_offset < in_len - 2 {
        hval = next(hval, data, current_offset);
        let hslot_idx = idx(hval);

        ref_offset = htab[hslot_idx];
        htab[hslot_idx] = current_offset;

        let off = current_offset.wrapping_sub(ref_offset).wrapping_sub(1);
        if off < MAX_OFF
            && current_offset + 4 < in_len
            && ref_offset > 0
            && ref_offset < in_len - 2
            && data[ref_offset] == data[current_offset]
            && data[ref_offset + 1] == data[current_offset + 1]
            && data[ref_offset + 2] == data[current_offset + 2]
        {
            let mut len = 2;
            let maxlen = cmp::min(in_len - current_offset - len, MAX_REF);

            /* stop run */
            out[(out_len - lit - 1) as usize] = (lit as u8).wrapping_sub(1);
            out_len -= not(lit); /* undo run if length is zero */

            if out_len as i32 + 3 + 1 >= out_buf_len as i32 {
                return Err(LzfError::NoCompressionPossible);
            }

            loop {
                len += 1;
                while len < maxlen && data[ref_offset + len] == data[current_offset + len] {
                    len += 1;
                }
                break;
            }

            len -= 2; /* len is now #octets - 1 */
            current_offset += 1;

            if len < 7 {
                out[out_len as usize] = (off >> 8) as u8 + (len << 5) as u8;
                out_len += 1;
            } else {
                out[out_len as usize] = (off >> 8) as u8 + (7 << 5);
                out[out_len as usize + 1] = (len as u8).wrapping_sub(7);
                out_len += 2;
            }

            out[out_len as usize] = off as u8;
            out_len += 2; /* start run */
            lit = 0;

            /* we add here, because we later substract from the total length */
            current_offset += len - 1;

            if current_offset >= in_len {
                break;
            }

            hval = first(data, current_offset);

            hval = next(hval, data, current_offset);
            htab[idx(hval)] = current_offset;
            current_offset += 1;

            hval = next(hval, data, current_offset);
            htab[idx(hval)] = current_offset;
            current_offset += 1;
        } else {
            /* one more literal byte we must copy */
            if out_len >= out_buf_len as i32 {
                return Err(LzfError::NoCompressionPossible);
            }

            lit += 1;
            out[out_len as usize] = data[current_offset];
            out_len += 1;
            current_offset += 1;

            if lit == MAX_LIT {
                /* stop run */
                out[(out_len - lit - 1) as usize] = (lit as u8).wrapping_sub(1);
                lit = 0;
                out_len += 1; /* start run */
            }
        }
    }

    /* at most 3 bytes can be missing here */
    if out_len + 3 > out_buf_len as i32 {
        return Err(LzfError::NoCompressionPossible);
    }

    while current_offset < in_len {
        lit += 1;
        out[out_len as usize] = data[current_offset];
        out_len += 1;
        current_offset += 1;

        if lit == MAX_LIT {
            /* stop run */
            out[(out_len - lit - 1) as usize] = (lit as u8).wrapping_sub(1);
            lit = 0;
            out_len += 1; /* start run */
        }
    }

    /* end run */
    out[(out_len - lit - 1) as usize] = (lit as u8).wrapping_sub(1);
    out_len -= not(lit); /* undo run if length is zero */

    unsafe { out.set_len(out_len as usize) };

    Ok(out)
}

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
    use super::decompress;

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

    match decompress(&compressed, lorem.len()) {
        Ok(decompressed) => {
            assert_eq!(lorem.len(), decompressed.len());
            assert_eq!(lorem.as_bytes(), &decompressed[..]);
        }
        Err(err) => panic!("Decompression failed with error {:?}", err),
    };
}

#[test]
fn test_alice_wonderland_both() {
    let alice = "\r\n\r\n\r\n\r\n                ALICE'S ADVENTURES IN WONDERLAND\r\n";

    let compressed = match compress(alice.as_bytes()) {
        Ok(c) => c,
        Err(err) => panic!("Compression failed with error {:?}", err),
    };

    let c_compressed = match super::compress(alice.as_bytes()) {
        Ok(c) => c,
        Err(err) => panic!("Compression failed with error {:?}", err),
    };

    assert_eq!(&compressed[..], &c_compressed[..]);
}

#[test]
fn quickcheck_found_bug() {
    let inp = vec![
        0, 0, 0, 0, 1, 0, 0, 2, 0, 0, 3, 0, 0, 4, 0, 1, 1, 0, 1, 2, 0, 1, 3, 0, 1, 4, 0, 0, 5, 0,
        0, 6, 0, 0, 7, 0, 0, 8, 0, 0, 9, 0, 0, 10, 0, 0, 11, 0, 1, 5, 0, 1, 6, 0, 1, 7, 0, 1, 8, 0,
        1, 9, 0, 1, 10, 0, 0,
    ];

    assert_eq!(LzfError::NoCompressionPossible, compress(&inp).unwrap_err());
}

#[test]
fn quickcheck_found_bug2() {
    let inp = vec![0];

    assert_eq!(LzfError::NoCompressionPossible, compress(&inp).unwrap_err());
}
