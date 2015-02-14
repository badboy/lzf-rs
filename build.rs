extern crate gcc;

fn main() {
    gcc::compile_library("liblzf.a", &["lzf/lzf_c.c", "lzf/lzf_d.c"]);
}
