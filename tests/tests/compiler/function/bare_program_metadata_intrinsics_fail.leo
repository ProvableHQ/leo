// FAIL: bare `_program_checksum`, `_program_edition`, `_program_owner`, and
// `_function_checksum` calls with a non-literal first argument used to slip past
// type-checking (the check was removed from the intrinsic handler when it moved to the
// `@_program_id_arg` wrapper site). The check has been restored on the intrinsic handler
// so bare misuse in user code is caught cleanly.
program test.aleo {
    fn bar() -> Final {
        return final { foo(); };
    }

    @noupgrade
    constructor() {}
}

export final fn foo() {
    let p: address = _self_address();
    let a: [u8; 32] = _program_checksum(p);
    let b: u16 = _program_edition(p);
    let c: address = _program_owner(p);
    let d: [u8; 32] = _function_checksum(p, 'bar');
    assert_neq(a, d);
    assert_eq(b, 0u16);
    assert_eq(c, c);
}
