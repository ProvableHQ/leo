final fn foo() {
    let c: [u8; 32] = self.checksum;
    let d: [u8; 32] = Program::checksum(test.aleo);
    let e: u16 = self.edition;
    let f: u16 = Program::edition(test.aleo);
    let g: address = self.program_owner;
    let h: address = Program::program_owner(test.aleo);
    let i: [u8; 32] = Program::function_checksum(test.aleo, 'bar');
    let j: [u8; 32] = Program::function_checksum(test.aleo, 'peek');
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
