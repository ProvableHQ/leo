/*
seed = 123456789

[case]
program = "test.aleo"
function = "outer"
input = ["1field"]
[case]

program = "test.aleo"
function = "outer_const"
input = ["17field"]
*/

program test.aleo {
    inline inner2(a: field) -> field {
        let b: field = 3field;
        return a + b;
    }

    function inner(a: field) -> field {
        let b: field = 2field;
        return inner2(a) + inner2(b);
    }

    transition outer(a: field) -> field {
        return inner(a);
    }

    inline inner_const::[X: field, Y: field](a: field) -> field {
        return X * (Y + a);
    }

    transition outer_const(x: field) -> field {
        return inner_const::[12field, 17field](x + 1field);
    }

    @noupgrade
    async constructor() {}
}
