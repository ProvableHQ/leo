/*
[case]
program = "test.aleo"
function = "foo"
input = []
*/

program child.aleo {
    transition dummy() {}
}

// --- Next Program --- //

import child.aleo;

program test.aleo {
    transition foo() {
        let a: address = Program::name_to_address("test.aleo");
        let b: address = Program::name_to_address("child.aleo");
        assert_neq(a, b);

        let c: [u8; 32] = Program::checksum("test.aleo");
        let d: [u8; 32] = Program::checksum("child.aleo");
        assert_neq(c, d);

        let e: u16 = Program::edition("test.aleo");
        let f: u16 = Program::edition("child.aleo");
        assert_eq(e, f);
    }
}
