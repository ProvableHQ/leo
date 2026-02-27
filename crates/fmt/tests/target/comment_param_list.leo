fn add(
    a: u64, // inline on a
    /* before b */
    b: u64,
) -> u64 {
    return a + b;
}

fn bar(
    a: u64,
    b: u64,
    // comment after last param
) -> u64 {
    return a + b;
}

program test.aleo {
    fn main(public a: u64, public b: u64) -> u64 {
        return add(a, b);
    }
}
