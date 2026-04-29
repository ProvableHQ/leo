// ANCHOR: const_generics_fn
fn sum_first_n_ints::[N: u32]() -> u32 {
    let sum = 0u32;
    for i: u32 in 0u32..N {
        sum += i;
    }
    return sum;
}

program main.aleo {
    fn main() -> u32 {
        return sum_first_n_ints::[5u32]();
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: const_generics_fn
