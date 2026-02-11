/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "some_assignments"
input = ["false", "false", "false"]
[case]
program = "test.aleo"
function = "some_assignments"
input = ["false", "false", "true"]
[case]
program = "test.aleo"
function = "some_assignments"
input = ["false", "true", "false"]
[case]
program = "test.aleo"
function = "some_assignments"
input = ["false", "true", "true"]
[case]
program = "test.aleo"
function = "some_assignments"
input = ["true", "false", "false"]
[case]
program = "test.aleo"
function = "some_assignments"
input = ["true", "false", "true"]
[case]
program = "test.aleo"
function = "some_assignments"
input = ["true", "true", "false"]
[case]
program = "test.aleo"
function = "some_assignments"
input = ["true", "true", "true"]
*/

program test.aleo {
    fn some_assignments(public a: bool, public b: bool, public c: bool) -> public [[u32; 2]; 2] {
        let arr: [[u32; 2]; 2] = [[1u32, 2u32], [3u32, 4u32]];
        if a {
            arr[0u8][0u8] = 10u32;
            arr[0u8][1u8] = 20u32;
        } else {
            arr[1u8][0u8] = 30u32;
        }
        if b {
            arr[0u8] = [100u32, 200u32];
        }
        if c {
            arr[0u8][0u8] = 1000u32;
            arr[0u8][1u8] = 2000u32;
        }
        return arr;
    }

    @noupgrade
    constructor() {}
}
