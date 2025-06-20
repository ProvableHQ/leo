/*
seed = 123456789

[case]
program = "test.aleo"
function = "main"
input = []
*/

program test.aleo {
    inline foo::[M: u32](arr: [u32; 3]) -> u32 {
        return arr[M - 1];
    }

    transition main() -> u32 {
        return foo::[3u32]([1, 2, 3]);
    }
}
