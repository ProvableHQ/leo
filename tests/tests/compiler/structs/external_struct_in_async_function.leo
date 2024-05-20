/*
namespace: Compile
expectation: Pass
*/

program parent.aleo {
    struct TestStruct {
        data0: u128,
        data1: u128
    }

    async transition init() -> Future {
        let test_struct: TestStruct = TestStruct {
            data0: 0u128,
            data1: 1u128
        };
        return finalize_init(test_struct);
    }

    async function finalize_init(public test_struct: TestStruct) {
        assert_eq(0u32, 0u32);
    }
}

// --- Next Program --- //

import parent.aleo;
program child.aleo {
    transition main() -> u32 {
        return 1u32;
    }
}