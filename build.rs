
fn main() {
    cc::Build::new()
        .cpp(true) // Switch to C++ library compilation.
        .file("src/helpers/call_rust.cpp")
        .compile("call_rust.lib");
    // println!("caca");
}

