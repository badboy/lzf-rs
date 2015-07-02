use super::{LzfResult,LzfError};

pub fn decompress(data: &[u8], out_len: usize) -> LzfResult<Vec<u8>> {
    let mut current_offset = 0;

    let mut output = Vec::with_capacity(out_len);

    let in_len = data.len();

    while current_offset < in_len {
        let mut ctrl = data[current_offset] as usize;
        current_offset += 1;

        if ctrl < (1<<5) {
            ctrl += 1;

            if current_offset+ctrl > in_len {
                return Err(LzfError::DataCorrupted);
            }

            while ctrl > 0 {
                output.push(data[current_offset]);
                current_offset += 1;
                ctrl -= 1;
            }
        } else {
            let mut len = ctrl >> 5;

            let mut ref_offset = (((ctrl & 0x1f) << 8) + 1) as i32;

            if len == 7 {
                len += data[current_offset] as usize;
                current_offset += 1;
            }

            ref_offset += data[current_offset] as i32;
            current_offset += 1;

            let mut ref_pos = (output.len() as i32) - ref_offset;
            if ref_pos < 0 {
                return Err(LzfError::DataCorrupted);
            }

            let c = output[ref_pos as usize];
            output.push(c);
            ref_pos += 1;

            let c = output[ref_pos as usize];
            output.push(c);
            ref_pos += 1;

            while len > 0 {
                let c = output[ref_pos as usize];
                output.push(c);
                ref_pos += 1;
                len -= 1;
            }
        }
    }

    Ok(output)
}

#[test]
fn test_native_decompress_lorem() {
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
}
