use std::io::Command;
use std::os;

fn main() {
    let out_dir = os::getenv("OUT_DIR").unwrap();

    // note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    Command::new("make").args(&["-C", "lzf"]).status().unwrap();

    println!("cargo:rustc-flags=-L {} -l lzf:static", out_dir);
}
