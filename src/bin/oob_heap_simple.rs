use std::slice;

fn main() {
    let mut v = Vec::new();
    v.push(10);

    let s = unsafe { slice::from_raw_parts(v.as_ptr(), 100) };

    println!("{}", s[99]);
}
