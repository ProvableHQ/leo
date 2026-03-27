// Test: an interface in one library inherits from an interface defined in another library.

// --- library: base_lib --- //

interface Base {
    fn fetch() -> u32;
}

// --- Next Program --- //

// --- library: ext_lib --- //

interface Extended : base_lib::Base {
    fn store(v: u32) -> u32;
}

// --- Next Program --- //

program main.aleo : ext_lib::Extended {
    fn fetch() -> u32 {
        return 42u32;
    }

    fn store(v: u32) -> u32 {
        return v + 1u32;
    }

    @noupgrade
    constructor() {}
}
