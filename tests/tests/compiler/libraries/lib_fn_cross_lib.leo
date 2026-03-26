// Test: a function in lib B calls a function from lib A.

// --- library: base_lib --- //

fn double(x: u32) -> u32 {
    return x + x;
}

// --- Next Program --- //

// --- library: derived_lib --- //

fn quadruple(x: u32) -> u32 {
    return base_lib::double(base_lib::double(x));
}

// --- Next Program --- //

program main.aleo {
    fn compute(x: u32) -> u32 {
        return derived_lib::quadruple(x);
    }

    @noupgrade
    constructor() {}
}
