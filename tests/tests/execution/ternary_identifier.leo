/*
seed = 123456789

[case]
program = "test.aleo"
function = "select"
input = ["true", "'foo'", "'bar'"]
[case]
program = "test.aleo"
function = "select"
input = ["false", "'foo'", "'bar'"]
[case]
program = "test.aleo"
function = "select_literal"
input = ["true"]
[case]
program = "test.aleo"
function = "select_literal"
input = ["false"]
*/

program test.aleo {
    fn select(s: bool, a: identifier, b: identifier) -> identifier {
        return s ? a : b;
    }

    fn select_literal(s: bool) -> identifier {
        return s ? 'foo' : 'bar';
    }

    @noupgrade
    constructor() {}
}
