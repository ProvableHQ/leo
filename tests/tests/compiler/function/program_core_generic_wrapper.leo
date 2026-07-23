// PASS: a user-defined generic wrapper around `std::prog::edition` uses a const generic
// parameter `P: address` as the first const argument. The program-ID literal check inside
// `visit_call` is skipped for non-literals (the reference is validated once monomorphisation
// substitutes the concrete literal and TypeChecking runs again).
program test.aleo {
    fn main(a: u16) -> Final {
        return final { finalize_main(a); };
    }

    @noupgrade
    constructor() {}
}

final fn wrap_edition::[P: address]() -> u16 {
    return std::prog::edition::[P]();
}

export final fn finalize_main(a: u16) {
    assert_eq(a, wrap_edition::[test.aleo]());
}
