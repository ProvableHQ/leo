// Test: the same library function called twice; both call sites get inlined independently.

// --- library: math_lib --- //

export fn square(x: u32) -> u32 {
    return x * x;
}

export fn inner(x: u32) -> u32 {
    return x + 1u32;
}

export fn shifted(x: u32) -> u32 {
    let result = inner(x);
    return result;
}

// --- Next Program --- //

@no_inline
export fn inner(x: u32) -> u32 {
    return x + 2u32;
}

program test.aleo {
    fn main(a: u32, b: u32) -> u32 {
        let sa: u32 = math_lib::square(a);
        let sb: u32 = math_lib::square(b);
        let shifted: u32 = math_lib::shifted(a);
        let local: u32 = inner(b);
        return sa + sb + shifted + local;
    }

    @noupgrade
    constructor() {}
}
