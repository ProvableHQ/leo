export final fn foo() {
    let c: [u8; 32] = std::prog::checksum::[test.aleo]();
    let d: [u8; 31] = std::prog::checksum::[credits.aleo]();
    assert_neq(c, d);

    let e: u16 = std::prog::edition::[d]();
    let f: u8 = std::prog::edition::[credits.aleo]();
    assert_neq(e, f);

    // The first argument must be a program ID.
    let g: [u8; 32] = std::prog::function_checksum::[d, 'bar']();
    // The function name must be an identifier literal, not a string.
    let h: [u8; 32] = std::prog::function_checksum::[test.aleo, "bar"]();
    // The function must resolve to an entry or view function; `missing` is neither.
    let i: [u8; 32] = std::prog::function_checksum::[test.aleo, 'missing']();
    assert_neq(g, h);
    assert_neq(g, i);
}

program test.aleo {
    fn bar() -> Final {
        return final { foo(); };
    }

    @noupgrade
    constructor() {}
}
