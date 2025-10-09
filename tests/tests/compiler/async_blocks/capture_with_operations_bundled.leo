program test.aleo {
    // Test that bundled variables can be used in complex expressions
    async transition main() -> Future {
        let a0 = 10u32;
        let a1 = 20u32;
        let a2 = 30u32;
        let a3 = 40u32;
        let a4 = 50u32;
        let a5 = 60u32;
        let a6 = 70u32;
        let a7 = 80u32;
        let a8 = 90u32;
        let a9 = 100u32;
        let b0 = 2u32;
        let b1 = 3u32;
        let b2 = 4u32;
        let b3 = 5u32;
        let b4 = 6u32;
        let b5 = 7u32;
        let b6 = 8u32;
        let b7 = 9u32;
        let c0 = 1u32;
        let c1 = 2u32;
        let c2 = 3u32;
        let c3 = 4u32;
        let c4 = 5u32;
        let c5 = 6u32;

        // 24 captures total - tests operations with bundled variables
        let f = async {
            let sum = a0 + a1 + a2 + a3 + a4 + a5 + a6 + a7 + a8 + a9;
            let product = b0 * b1 * b2 * b3;
            let mixed = (sum / b4) + (product - b5) + b6 + b7;
            let extra = c0 + c1 + c2 + c3 + c4 + c5;
            assert_eq(sum, 550u32);
            assert_eq(product, 120u32);
            assert_eq(extra, 21u32);
            assert(mixed > 0u32);
        };
        return f;
    }
}

