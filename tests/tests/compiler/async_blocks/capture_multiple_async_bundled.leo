program test.aleo {
    // Test multiple async transition functions with bundling
    // Ensures synthetic struct names don't collide across different functions
    async transition first() -> Future {
        // First set of variables for first async block
        let a0 = 0u32;
        let a1 = 1u32;
        let a2 = 2u32;
        let a3 = 3u32;
        let a4 = 4u32;
        let a5 = 5u32;
        let a6 = 6u32;
        let a7 = 7u32;
        let a8 = 8u32;
        let a9 = 9u32;
        let a10 = 10u32;
        let a11 = 11u32;
        let a12 = 12u32;
        let a13 = 13u32;
        let a14 = 14u32;
        let a15 = 15u32;
        let a16 = 16u32;

        // Async block with 17 captures
        let f = async {
            let sum1 = a0 + a1 + a2 + a3 + a4 + a5 + a6 + a7 + a8 + a9
                     + a10 + a11 + a12 + a13 + a14 + a15 + a16;
            assert_eq(sum1, 136u32);
        };
        return f;
    }

    async transition second() -> Future {
        // Second set of variables for second async transition
        let b0 = 20u32;
        let b1 = 21u32;
        let b2 = 22u32;
        let b3 = 23u32;
        let b4 = 24u32;
        let b5 = 25u32;
        let b6 = 26u32;
        let b7 = 27u32;
        let b8 = 28u32;
        let b9 = 29u32;
        let b10 = 30u32;
        let b11 = 31u32;
        let b12 = 32u32;
        let b13 = 33u32;
        let b14 = 34u32;
        let b15 = 35u32;
        let b16 = 36u32;
        let b17 = 37u32;

        // Second async block with 18 captures
        let f = async {
            let sum2 = b0 + b1 + b2 + b3 + b4 + b5 + b6 + b7 + b8 + b9
                     + b10 + b11 + b12 + b13 + b14 + b15 + b16 + b17;
            assert_eq(sum2, 513u32);
        };
        return f;
    }
}

