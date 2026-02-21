/*
seed = 123456789
min_height = 16

[case]
program = "vec_program_c.aleo"
function = "combine"
input = []
*/


program vec_program_a.aleo {
    storage numbers: [u32];

    fn push_a() -> Final {
        return final {
            numbers.push(10u32);
            numbers.push(20u32);
        };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

program vec_program_b.aleo {
    storage numbers: [u32];

    fn push_b() -> Final {
        return final {
            numbers.push(1u32);
            numbers.push(2u32);
            numbers.push(3u32);
        };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import vec_program_a.aleo;
import vec_program_b.aleo;

program vec_program_c.aleo {
    storage sum_vec: [u32];

    fn combine() -> Final {
        let f_a: Final = vec_program_a.aleo/push_a();
        let f_b: Final = vec_program_b.aleo/push_b();

        return final {
            f_a.run();
            f_b.run();

            let len_a: u32 = vec_program_a.aleo/numbers.len();
            let len_b: u32 = vec_program_b.aleo/numbers.len();

            assert(len_a == 2);
            assert(len_b == 3);

            const max_len:u32 = 3;

            sum_vec.clear();

            for i:u32 in 0..max_len {
                let a_val = vec_program_a.aleo/numbers.get(i).unwrap_or(0u32);
                let b_val = vec_program_b.aleo/numbers.get(i).unwrap_or(0u32);
                sum_vec.push(a_val + b_val);
            }

            // Assertions
            assert(sum_vec.get(0).unwrap() == 10u32 + 1);
            assert(sum_vec.get(1).unwrap() == 20u32 + 2);
            assert(sum_vec.get(2).unwrap() == 0u32 + 3);
        };
    }

    @noupgrade
    constructor() {}
}
