/*
seed = 123456789

[case]
program = "test_ternary.aleo"
function = "do_ternary"
input = ["true", "true", "1field", "2field", "3field"]
[case]
program = "test_ternary.aleo"
function = "do_ternary"
input = ["true", "false", "1field", "2field", "3field"]
[case]
program = "test_ternary.aleo"
function = "do_ternary"
input = ["false", "true", "1field", "2field", "3field"]
[case]
program = "test_ternary.aleo"
function = "do_ternary"
input = ["false", "false", "1field", "2field", "3field"]
*/

program test_ternary.aleo {
    struct S {
        a: field,
    }

    transition do_ternary(x: bool, y: bool, f1: field, f2: field, f3: field) -> S {
        let s1 = S { a: f1 };
        let s2 = S { a: f2 };
        let s3 = S { a: f3 };
        return x ? (y ? s1 : s2) : s2;
    }

    @noupgrade
    async constructor() {}
}
