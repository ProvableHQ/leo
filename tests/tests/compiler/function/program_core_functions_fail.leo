
program test.aleo {
    async function foo() {
        let c: [u8; 32] = Program::checksum(test.aleo);
        let d: [u8; 31] = Program::checksum(credits.aleo);
        assert_neq(c, d);

        let e: u16 = Program::edition(d);
        let f: u8 = Program::edition(credits.aleo);
        assert_neq(e, f);
    }

    async transition bar() -> Future {
        return foo();
    }
}
