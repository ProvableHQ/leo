program test.aleo {
    async function foo() {
        let a: address = Program::name_to_address("test.aleo");
        let b: address = Program::name_to_address("credits.aleo");
        let c: [u8; 32] = Program::checksum("test.aleo");
        let d: [u8; 32] = Program::checksum("credits.aleo");
        let e: u16 = Program::edition("test.aleo");
        let f: u16 = Program::edition("credits.aleo");
        let g: address = Program::program_owner("test.aleo");
        let h: address = Program::program_owner("credits.aleo");
        assert_eq(a, b);
        assert_eq(c, d);
        assert_eq(e, f);
        assert_eq(g, h);
    }

    async transition bar() -> Future {
      return foo();
    }
}
