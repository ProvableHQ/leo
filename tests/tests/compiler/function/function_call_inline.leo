
program test.aleo {
    fn main(a: u32, b: u32, y: bool) -> u32 {
        check_not_eq(a, b);
        if y {
           return adder(a, b);
        } else {
            return subber(a, b);
        }
    }
}

@inline
fn check_not_eq(a: u32, b: u32) {
    assert(a != b);
}

@inline
fn adder(a: u32, b: u32) -> u32 {
    return a + b;
}

@inline
fn subber(a: u32, b: u32) -> u32 {
    return a - b;
}
