use std::process::Command;
use std::path::Path;

fn main() {
    Command::new("gmake")
        .args(&["clean"])
        .current_dir(&Path::new("./native"))
        .status()
        .unwrap();

    Command::new("gmake")
        .args(&["static"])
        .current_dir(&Path::new("./native"))
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", "./native");
    println!("cargo:rustc-link-lib=static=hello");
}