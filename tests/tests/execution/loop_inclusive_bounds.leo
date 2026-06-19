/*
seed = 123456789

[case]
program = "test.aleo"
function = "inclusive_equal"
input = []
[case]
program = "test.aleo"
function = "exclusive_equal"
input = []
[case]
program = "test.aleo"
function = "inclusive_decreasing"
input = []
[case]
program = "test.aleo"
function = "inclusive_equal_assert"
input = []
*/

program test.aleo {
    fn inclusive_equal() -> u32 {
        let count = 0u32;
        for i in 5u32..=5u32 {
            count += 1u32;
        }
        return count;
    }

    fn exclusive_equal() -> u32 {
        let count = 0u32;
        for i in 5u32..5u32 {
            count += 1u32;
        }
        return count;
    }

    fn inclusive_decreasing() -> u32 {
        let count = 0u32;
        for i in 6u32..=5u32 {
            count += 1u32;
        }
        return count;
    }

    fn inclusive_equal_assert() -> u32 {
        for i in 5u32..=5u32 {
            assert(false);
        }
        return 0u32;
    }

    @noupgrade
    constructor() {}
}
