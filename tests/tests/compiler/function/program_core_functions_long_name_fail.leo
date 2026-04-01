
final fn foo() {
    let c: [u8; 32] = Program::checksum(aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aleo);
    let d: u16 = Program::edition(aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aleo);
    let e: address = Program::program_owner(aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aleo);
}

program test.aleo {
    fn bar() -> Final {
        return final { foo(); };
    }
}
