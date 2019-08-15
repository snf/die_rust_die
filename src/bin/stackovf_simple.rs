#[allow(unconditional_recursion)]
fn recurse(n: u32) -> u32 {
    return recurse(n+1);
}

fn main() {
    recurse(0);
}