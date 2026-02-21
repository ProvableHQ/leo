final fn foo() {
    let c: [u8; 32] = self.checksum;
    let d: [u8; 32] = Program::checksum(test.aleo);
    let e: u16 = self.edition;
    let f: u16 = Program::edition(test.aleo);
    let g: address = self.program_owner;
    let h: address = Program::program_owner(test.aleo);
    assert_eq(c, d);
    assert_eq(e, f);
    assert_eq(g, h);
}

program test.aleo {
    fn bar() -> Final {
      return final { foo(); };
    }
}
