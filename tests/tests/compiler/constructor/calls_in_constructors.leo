export fn check_first_entry_is_zero(entry: u8) {
    assert_eq(entry, 0u8);
}

program test.aleo {
    mapping foo: u8 => u8;

    fn food() {}

    @custom
    constructor() {
        let entry: u8 = foo.get(0u8);
        check_first_entry_is_zero(entry);
        checks::check_edition::[1u16]();
    }
}

// --- Next Module: checks.leo --- //

export final fn check_edition::[N: u16]() {
    assert_neq(std::ctx::edition(), N);
}

// --- Next Program --- //

import test.aleo;

program child.aleo {
    fn main() {}

    @custom
    constructor() {
        test.aleo::checks::check_edition::[2u16]();
    }
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    fn main() {}

    @noupgrade
    constructor() {}
}
