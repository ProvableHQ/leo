// Test: a library function and a main-program function each independently use
// `Optional<u32>` internally, without crossing Optional across the unit
// boundary.
//
// Exercises per-unit placement of generated wrappers: each `Optional<u32>`
// wrapper is keyed into `self.composites` under its owning unit so that
// `items_at_path` pulls the library's wrapper into the library and the main
// program's wrapper into the program, with neither leaking into the other.

// --- library: math_lib --- //

fn double_default(v: u32) -> u32 {
    let maybe: u32? = none;
    return maybe.unwrap_or(v) * 2u32;
}

// --- Next Program --- //

program twofold.aleo {
    fn compute(v: u32) -> u32 {
        let maybe: u32? = none;
        let base = maybe.unwrap_or(v);
        return math_lib::double_default(base);
    }

    @noupgrade
    constructor() {}
}
