/*
seed = 123456789

[case]
program = "caller_cross.aleo"
function = "call_store"
input = ["5u32"]

[case]
program = "caller_cross.aleo"
function = "call_fetch"
input = []
*/

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

// impl_cross.aleo implements ext_lib::Extended, which inherits base_lib::Base,
// so it must provide both `store` and `fetch`.
program impl_cross.aleo : ext_lib::Extended {
    fn fetch() -> u32 {
        return 42u32;
    }

    fn store(v: u32) -> u32 {
        return v + 1u32;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import impl_cross.aleo;

program caller_cross.aleo {
    // Dispatch through the child interface (ext_lib::Extended) for `store`.
    fn call_store(v: u32) -> u32 {
        return ext_lib::Extended@('impl_cross')/store(v);
    }

    // Dispatch through the parent interface (base_lib::Base) for `fetch`.
    fn call_fetch() -> u32 {
        return base_lib::Base@('impl_cross')/fetch();
    }

    @noupgrade
    constructor() {}
}
