extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .include("Tomato/include")
        .include("Tomato/source")
        .file("Tomato/source/Export.cpp")
        .file("src/helper.cpp")
        .compile("libcode.a");
}