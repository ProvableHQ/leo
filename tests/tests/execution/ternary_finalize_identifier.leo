/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "update"
input = ["true", "'alpha'", "'beta'"]
[case]
program = "test.aleo"
function = "update"
input = ["false", "'alpha'", "'beta'"]
[case]
program = "test.aleo"
function = "update_literal"
input = ["true"]
[case]
program = "test.aleo"
function = "update_literal"
input = ["false"]
*/

program test.aleo {
    mapping names: u32 => identifier;

    fn update(s: bool, a: identifier, b: identifier) -> Final {
        return final {
            let chosen: identifier = s ? a : b;
            names.set(0u32, chosen);
        };
    }

    fn update_literal(s: bool) -> Final {
        return final {
            let chosen: identifier = s ? 'foo' : 'bar';
            names.set(1u32, chosen);
        };
    }

    @noupgrade
    constructor() {}
}
