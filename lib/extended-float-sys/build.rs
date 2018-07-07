extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/extended.c")
        .flag("-Wall").flag("-Werror")
        .flag("-O1")
        .compile("toymath");
}