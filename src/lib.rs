use std::slice;

#[inline(always)]
pub fn uaf_vec() {
    let p = {
        let mut v = Vec::new();
        v.push(10);
        v.as_ptr()
    };

    let s = unsafe { slice::from_raw_parts(p, 1)};
    println!("{}", s[0]);
}

#[allow(unconditional_recursion)]
pub fn recurse_infinite(n: u32) -> u32 {
    return recurse_infinite(n+1);
}


#[inline(always)]
pub fn heap_oob_read() {
    let mut v = Vec::new();
    v.push(10);

    let s = unsafe { slice::from_raw_parts(v.as_ptr(), 100) };

    println!("{}", s[99]);
}