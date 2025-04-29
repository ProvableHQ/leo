/*
seed = 123456789

[case]
program = "test.aleo"
function = "main"
input = ["1u64", "2u64", "3u64", "4u64", "true", "true", "true"]
[case]
program = "test.aleo"
function = "main"
input = ["1u64", "2u64", "3u64", "4u64", "false", "true", "true"]
[case]
program = "test.aleo"
function = "main"
input = ["1u64", "2u64", "3u64", "4u64", "false", "true", "false"]
[case]
program = "test.aleo"
function = "main"
input = ["1u64", "2u64", "3u64", "4u64", "false", "false", "true"]
[case]
program = "test.aleo"
function = "main"
input = ["1u64", "2u64", "3u64", "4u64", "false", "false", "false"]
*/

program test.aleo {
    transition main(a: u64, b: u64, c: u64, d: u64, y: bool, w: bool, z: bool) -> (public u64, public u64, public u64, public u64) {
        let tuple: (u64, u64, u64, u64) = (a, b, c, d);

        if y {
            if w {
                tuple.0 = 100u64;
                tuple.1 = 200u64;
            } else {
                tuple.2 = 300u64;
            }
            tuple.3 = 400u64;
        } else if z {
            if w {
                tuple.0 = 500u64;
                tuple.2 = 600u64;
            } else {
                tuple.1 = 700u64;
            }
        } else {
            tuple.3 = 800u64;
        }

        return tuple;
    }
}
