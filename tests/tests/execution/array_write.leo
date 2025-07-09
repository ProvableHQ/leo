/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "some_assignments"
input = ["false", "false"]
[case]
program = "test.aleo"
function = "some_assignments"
input = ["false", "true"]
[case]
program = "test.aleo"
function = "some_assignments"
input = ["true", "false"]
[case]
program = "test.aleo"
function = "some_assignments"
input = ["true", "true"]
*/


program test.aleo {
    struct S {
        x: u32,
        y: [u32; 3],
    }

    transition some_assignments(public a: bool, public b: bool) -> public [u32; 11] {
        let arr: [u32; 3] = [0u32, 1u32, 2u32];
        if a {
            arr[0u8] = 10u32;
        }

        let s: S = S {
            x: 3u32,
            y: arr,
        };

        let tuple: (S, S) = (s, s);

        if b {
            tuple.0.y[0u8] = 20u32;
            tuple.1.x = 30u32;
            if a {
                tuple.0 = s;
                tuple.1.y = [50u32, 60u32, 70u32];
            }
        }

        return [
            arr[0u8],
            arr[1u8],
            arr[2u8],
            tuple.0.x,
            tuple.0.y[0u8],
            tuple.0.y[1u8],
            tuple.0.y[2u8],
            tuple.1.x,
            tuple.1.y[0u8],
            tuple.1.y[1u8],
            tuple.1.y[2u8],
        ];
        
    }
}
