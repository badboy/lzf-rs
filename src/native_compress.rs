use super::{LzfResult,LzfError};
use std::cmp;

const HLOG    : usize = 16;
const HSIZE   : u32 = 1 << HLOG;
const MAX_OFF : usize = 1 << 13;
const MAX_REF : usize = ((1 << 8) + (1 << 3));
const MAX_LIT : i32 = (1 << 5);

#[inline(always)]
fn get(d: &[u8], i: usize) -> u8 {
    unsafe { *d.get_unchecked(i) }
}

#[inline(always)]
fn first(p: &[u8], off: usize) -> u32 {
    ((get(p,off) as u32) << 8) | get(p,off+1) as u32
}

#[inline(always)]
fn next(v: u32, p: &[u8], off: usize) -> u32 {
    (v << 8) | get(p,off+2) as u32
}

#[inline(always)]
fn idx(h: u32) -> usize {
    let h = h as u64;
    (
        // 8 = 3*8-HLOG, but HLOG is constant at 16
        (h.wrapping_shr(8).wrapping_sub(h*5))
        & (HSIZE-1) as u64
    ) as usize
}

#[inline(always)]
fn not(i: i32) -> i32 {
    if i == 0 { 1 } else { 0 }
}

pub fn compress(data: &[u8]) -> LzfResult<Vec<u8>> {
    let in_len = data.len();
    let out_buf_len = in_len;
    let mut out = Vec::with_capacity(out_buf_len);
    unsafe { out.set_len(out_buf_len) };

    let mut out_len : i32 = 0;

    let mut htab = [0; 1<<HLOG];

    let mut current_offset = 0;

    if in_len == 0 {
        return Err(LzfError::DataCorrupted);
    }

    let mut lit : i32 = 0;

    out_len += 1;

    let mut hval : u32;
    let mut ref_offset;

    hval = first(data, current_offset);

    while current_offset < in_len-2 {
        hval = next(hval, data, current_offset);
        let hslot_idx = idx(hval);

        unsafe {
            ref_offset = *htab.get_unchecked(hslot_idx);
            *htab.get_unchecked_mut(hslot_idx) = current_offset;
        }

        let off = current_offset.wrapping_sub(ref_offset).wrapping_sub(1);
        if off < MAX_OFF
            && current_offset+4 < in_len
            && ref_offset > 0
            && get(data,ref_offset+0) == get(data,current_offset+0)
            && get(data,ref_offset+1) == get(data,current_offset+1)
            && get(data,ref_offset+2) == get(data,current_offset+2) {

            let mut len = 2;
            let maxlen = cmp::min(in_len - current_offset - len, MAX_REF);

            unsafe { *out.get_unchecked_mut((out_len - lit - 1) as usize) = (lit as u8).wrapping_sub(1); }
            out_len -= not(lit);

            if out_len as i32 + 3 + 1 >= out_buf_len as i32 {
                return Err(LzfError::NoCompressionPossible);
            }

            loop {
                // Unrool loop.
                if maxlen > 16 {
                    len += 1; if get(data,ref_offset+len) != get(data,current_offset+len) { break; }
                    len += 1; if get(data,ref_offset+len) != get(data,current_offset+len) { break; }
                    len += 1; if get(data,ref_offset+len) != get(data,current_offset+len) { break; }
                    len += 1; if get(data,ref_offset+len) != get(data,current_offset+len) { break; }

                    len += 1; if get(data,ref_offset+len) != get(data,current_offset+len) { break; }
                    len += 1; if get(data,ref_offset+len) != get(data,current_offset+len) { break; }
                    len += 1; if get(data,ref_offset+len) != get(data,current_offset+len) { break; }
                    len += 1; if get(data,ref_offset+len) != get(data,current_offset+len) { break; }

                    len += 1; if get(data,ref_offset+len) != get(data,current_offset+len) { break; }
                    len += 1; if get(data,ref_offset+len) != get(data,current_offset+len) { break; }
                    len += 1; if get(data,ref_offset+len) != get(data,current_offset+len) { break; }
                    len += 1; if get(data,ref_offset+len) != get(data,current_offset+len) { break; }

                    len += 1; if get(data,ref_offset+len) != get(data,current_offset+len) { break; }
                    len += 1; if get(data,ref_offset+len) != get(data,current_offset+len) { break; }
                    len += 1; if get(data,ref_offset+len) != get(data,current_offset+len) { break; }
                    len += 1; if get(data,ref_offset+len) != get(data,current_offset+len) { break; }
                }

                len += 1;
                while len < maxlen && get(data,ref_offset+len) == get(data,current_offset+len) {
                    len += 1;
                }
                break;
            }

            len -= 2;
            current_offset += 1;

            if len < 7 {
                unsafe{ *out.get_unchecked_mut(out_len as usize) = (off >> 8) as u8 + (len << 5) as u8; }
                out_len += 1;
            } else {
                unsafe {
                    *out.get_unchecked_mut(out_len as usize) = (off >> 8) as u8 + (7 << 5);
                    *out.get_unchecked_mut(out_len as usize + 1) = (len as u8).wrapping_sub(7);
                }
                out_len += 2;
            }

            unsafe { *out.get_unchecked_mut(out_len as usize) = off as u8; }
            out_len += 2;
            lit = 0;

            current_offset += len-1;

            if current_offset >= in_len {
                break;
            }

            hval = first(data, current_offset);

            hval = next(hval, data, current_offset);
            unsafe { *htab.get_unchecked_mut(idx(hval)) = current_offset; }
            current_offset += 1;

            hval = next(hval, data, current_offset);
            unsafe { *htab.get_unchecked_mut(idx(hval)) = current_offset; }
            current_offset += 1;
        } else {
            if current_offset >= out_buf_len {
                return Err(LzfError::NoCompressionPossible);
            }

            lit += 1;
            unsafe { *out.get_unchecked_mut(out_len as usize) = get(data,current_offset); }

            out_len += 1;
            current_offset += 1;

            if lit == MAX_LIT {
                unsafe { *out.get_unchecked_mut((out_len - lit - 1) as usize) = (lit as u8).wrapping_sub(1); }
                lit = 0;
                out_len += 1;
            }
        }
    }

    if out_len + 3 > out_buf_len as i32 {
        return Err(LzfError::NoCompressionPossible);
    }

    while current_offset < in_len {
        lit += 1;
        unsafe { *out.get_unchecked_mut(out_len as usize) = get(data,current_offset); }
        out_len += 1;
        current_offset += 1;

        if lit == MAX_LIT {
            unsafe { *out.get_unchecked_mut((out_len - lit - 1) as usize) = (lit as u8).wrapping_sub(1); }
            lit = 0;
            out_len += 1;
        }
    }

    unsafe { *out.get_unchecked_mut((out_len - lit - 1) as usize) = (lit as u8).wrapping_sub(1); }
    out_len -= not(lit);

    unsafe { out.set_len(out_len as usize) };

    Ok(out)
}

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
    use super::native;

    let lorem = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.";

    let compressed = match compress(lorem.as_bytes()) {
        Ok(c) => c,
        Err(err) => panic!("Compression failed with error {:?}", err)
    };

    match native::decompress(&compressed, lorem.len()) {
        Ok(decompressed) => {
            assert_eq!(lorem.len(), decompressed.len());
            assert_eq!(lorem.as_bytes(), &decompressed[..]);
        },
        Err(err) => panic!("Decompression failed with error {:?}", err)
    };
}
