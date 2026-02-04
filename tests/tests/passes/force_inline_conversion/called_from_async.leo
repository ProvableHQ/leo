// Functions called from an async block should be converted to inline
program test.aleo {
    mapping counter: address => u64;

    fn main(owner: address) -> Final {
        return final {
            let value: u64 = compute(5u64);
            counter.set(owner, value);
        };
    }
}

// compute is called from an async block, so it should become inline
fn compute(x: u64) -> u64 {
    return x * 2u64;
}
