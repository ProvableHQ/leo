/*
seed = 123456789

[case]
program = "test_tuples.aleo"
function = "do_tuples"
input = ["1234field", "12u8", "true"]
[case]
program = "test_tuples.aleo"
function = "do_tuples"
input = ["123field", "1u8", "false"]
*/


program test_tuples.aleo {
    transition do_tuples(x: field, y: u8, z: bool) -> (field, [u8; 2]) {
        let t1 = (x, [y, y]);
        if z {
            t1.0 += 1;
            t1.1[0u32] += 1u8;
        }

        return t1;
    }

    @noupgrade
    async constructor() {}
}
