
struct TestStruct {
    data0: u128,
    data1: u128
}

program parent.aleo {
    fn init() -> Final {
        let test_struct: TestStruct = TestStruct {
            data0: 0u128,
            data1: 1u128
        };
        return final { finalize_init(test_struct); };
    }
}

final fn finalize_init(public test_struct: TestStruct) {
    assert_eq(0u32, 0u32);
}

// --- Next Program --- //

import parent.aleo;
program child.aleo {
    fn main() -> u32 {
        return 1u32;
    }
}
