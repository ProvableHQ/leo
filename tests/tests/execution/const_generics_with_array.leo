/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "main"
input = []
*/

inline foo::[M: u32](arr: [u32; 3]) -> u32 {
    return arr[M - 1];
}

program test.aleo {
    transition main() -> u32 {
        return foo::[3u32]([1, 2, 3]);
    }

    @noupgrade
    async constructor() {}
}
