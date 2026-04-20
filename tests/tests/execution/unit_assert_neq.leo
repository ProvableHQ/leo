/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "main"
input = ["5u32"]
*/

// Tests that assert_neq on two unit-returning expressions compiles to an
// instruction that always fails (Unit != Unit is trivially false), so the
// finalize is always rejected.
program test.aleo {
    mapping values: u32 => u32;

    fn main(public a: u32) -> Final {
        return final {
            assert_neq(fin_foo(a), fin_foo(a));
        };
    }

    @noupgrade
    constructor() {}
}

final fn fin_foo(a: u32) {
    Mapping::set(values, a, a);
}
