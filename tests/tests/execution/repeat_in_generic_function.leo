/*
seed = 123456789

[case]
program = "test.aleo"
function = "main"
input = []
*/

program test.aleo {
    const N:u32 = 10;
    transition main() -> u32 {
        let sum = 0u32;
        for i: u32 in 1..N {
            sum += foo::[i]();
        }
        return sum;
    }

    inline foo::[P: u32]() -> u32 {
        let arr = [P; P];
        return arr[P-1];
    }
}
