use std::slice;
// use rust_crash::uaf;

fn main() {
    let p = {
        let mut v = Vec::new();
        v.push(10);
        v.as_ptr()
    };

    let s = unsafe { slice::from_raw_parts(p, 1)};
    println!("{}", s[0]);
}
