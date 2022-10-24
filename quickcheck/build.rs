fn main() {
    cc::Build::new()
        .file("lzf/lzf_c.c")
        .file("lzf/lzf_d.c")
        .compile("lzf");
}
