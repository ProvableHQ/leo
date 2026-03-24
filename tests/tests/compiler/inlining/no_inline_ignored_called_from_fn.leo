@no_inline
fn inner(x: u32) -> u32 {
    return x + 1u32;
}

fn outer(x: u32) -> u32 {
    return inner(x);
}

program test.aleo {
    fn main(x: u32) -> u32 {
        return outer(x);
    }
}
