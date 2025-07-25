program test.aleo {
    async function foo() {
        let c: [u8; 32] = self.checksum;
        let d: [u8; 32] = Program::checksum(credits.aleo);
        let e: u16 = self.edition;
        let f: u16 = Program::edition(credits.aleo);
        let g: address = self.program_owner;
        let h: address = Program::program_owner(credits.aleo);
        assert_eq(a, b);
        assert_eq(c, d);
        assert_eq(e, f);
        assert_eq(g, h);
    }

    async transition bar() -> Future {
      return foo();
    }
}
