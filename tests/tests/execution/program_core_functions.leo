/*
seed = 123456789

[case]
program = "test.aleo"
function = "bar"
input = []
*/

program child.aleo {
    fn dummy() {}

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import child.aleo;

program test.aleo {
    fn bar() -> Final {
        return final {
            let c: [u8; 32] = self.checksum;
            let d: [u8; 32] = Program::checksum(child.aleo);
            assert_neq(c, d);

            let e: u16 = Program::edition(test.aleo);
            let f: u16 = Program::edition(child.aleo);
            assert_eq(e, f);

            let g: address = Program::program_owner(test.aleo);
            let h: address = Program::program_owner(child.aleo);
            assert_eq(g, h);
        };
    }

    @noupgrade
    constructor() {}
}
