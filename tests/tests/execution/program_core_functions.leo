/*
[case]
program = "test.aleo"
function = "bar"
input = []

[case]
program = "test.aleo"
function = "baz"
input = []
*/

program child.aleo {
    transition dummy() {}

    @noupgrade
    async constructor() {}
}

// --- Next Program --- //

import child.aleo;

program test.aleo {
    async function foo() {
        let c: [u8; 32] = self.checksum;
        let d: [u8; 32] = Program::checksum(child.aleo);
        assert_neq(c, d);

        let e: u16 = Program::edition(test.aleo);
        let f: u16 = Program::edition(child.aleo);
        assert_eq(e, f);
    }

    async function qux() {
        // This should fail because `credits.aleo` does not have a `program_owner`.
        assert_neq(Program::program_owner(credits.aleo), self.program_owner);
    }

    async transition bar() -> Future {
        return foo();
    }

    async transition baz() -> Future {
        return qux();
    }

    @noupgrade
    async constructor() {}
}
