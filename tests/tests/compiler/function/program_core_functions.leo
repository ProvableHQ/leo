export final fn foo() {
    let c: [u8; 32] = std::ctx::checksum();
    let d: [u8; 32] = std::prog::checksum::[test.aleo]();
    let e: u16 = std::ctx::edition();
    let f: u16 = std::prog::edition::[test.aleo]();
    let g: address = std::ctx::program_owner();
    let h: address = std::prog::program_owner::[test.aleo]();
    let i: [u8; 32] = std::prog::function_checksum::[test.aleo, 'bar']();
    let j: [u8; 32] = std::prog::function_checksum::[test.aleo, 'peek']();
    assert_eq(c, d);
    assert_eq(e, f);
    assert_eq(g, h);
    assert_neq(i, j);
}

program test.aleo {
    mapping vals: u32 => u32;

    view fn peek(k: u32) -> u32 {
        return vals.get_or_use(k, 0u32);
    }

    fn bar() -> Final {
        return final { foo(); };
    }

    @noupgrade
    constructor() {}
}
