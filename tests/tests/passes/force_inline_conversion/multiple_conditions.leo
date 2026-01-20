// Test: Multiple functions with different conditions
program test.aleo {
    mapping counter: address => u64;

    fn main(owner: address, x: u32) -> Future {
        let a: u32 = outer(x);
        return async {
            let value: u64 = async_helper(5u64);
            counter.set(owner, value);
        };
    }
}

// outer is a Function that calls inner - outer stays fn, inner becomes inline
fn outer(x: u32) -> u32 {
    return inner(x);
}

// inner is called by outer (a Function) - should become inline
fn inner(x: u32) -> u32 {
    return x + 1u32;
}

// async_helper is called from async block - should become inline
fn async_helper(x: u64) -> u64 {
    return x * 2u64;
}

// no_args has no arguments - should become inline
fn no_args() -> u32 {
    return 100u32;
}

// with_const has const parameters - should become inline
fn with_const::[N: u32](x: u32) -> u32 {
    return x * N;
}

// regular has arguments, no const params, not called from Function or async - stays fn
fn regular(x: u32) -> u32 {
    return x;
}
