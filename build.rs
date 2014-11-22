extern crate gcc;
use std::default::Default;

fn main() {
    gcc::compile_library("liblzf.a",
                         &Default::default(),
                         &["lzf/lzf_c.c", "lzf/lzf_d.c"]);
}
