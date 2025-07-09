/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "main"
input = []
*/

program test.aleo {
    const M: u32 = 1u32;
    const P: u32 = 2u32;

    transition main() -> u32 {
        let sum = 0u32;
        for i in 0u32..5u32 {
            sum += sum_first_n_ints::[M + i]() + sum_first_n_ints::[P + i]();
        }

        return  sum
              + sum_first_n_ints::[2 * 3]()
              + other::[5](); // this call to `other` will get resolved in the first pass of monomorphization
    }

    inline sum_first_n_ints::[N: u32]() -> u32 {
        let sum = 0u32;
        for i in 0u32..N {
            sum += i + other::[i](); // this call to `other` will be resolved in the second pass of monomorphization
        }
        return sum;
    }

    inline other::[Q: u32]() -> u32 {
        return Q;
    }
}
