/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "main"
input = []
*/

const N:u32 = 10;


fn foo::[P: u32]() -> u32 {
    let arr = [P; P];
    return arr[P-1];
}

program test.aleo {
    fn main() -> u32 {
        let sum = 0u32;
        for i: u32 in 1..N {
            sum += foo::[i]();
        }
        return sum;
    }

    @noupgrade
    constructor() {}
}
