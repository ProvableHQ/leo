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
            let c: [u8; 32] = std::ctx::checksum();
            let d: [u8; 32] = std::prog::checksum::[child.aleo]();
            assert_neq(c, d);

            let e: u16 = std::prog::edition::[test.aleo]();
            let f: u16 = std::prog::edition::[child.aleo]();
            assert_eq(e, f);

            let g: address = std::prog::program_owner::[test.aleo]();
            let h: address = std::prog::program_owner::[child.aleo]();
            assert_eq(g, h);

            let i: [u8; 32] = std::prog::function_checksum::[test.aleo, 'bar']();
            let j: [u8; 32] = std::prog::function_checksum::[child.aleo, 'dummy']();
            assert_neq(i, j);
        };
    }

    @noupgrade
    constructor() {}
}
