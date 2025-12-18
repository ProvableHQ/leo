program test.aleo {
    // Test capturing tuple fields that require bundling
    // Tuple fields count as individual captures
    async transition main() -> Future {
        let tuple1 = (1u32, 2u32, 3u32, 4u32);
        let tuple2 = (5u32, 6u32, 7u32, 8u32);
        let tuple3 = (9u32, 10u32, 11u32, 12u32);
        let tuple4 = (13u32, 14u32, 15u32, 16u32);
        let tuple5 = (17u32, 18u32, 19u32, 20u32);
        let tuple6 = (21u32, 22u32, 23u32);
        let a = 100u32;
        let b = 200u32;

        // Captures: 4+4+4+4+4+3 tuple fields + 2 scalars = 25 total
        // Should create 1 bundle with 9 fields
        let f = async {
            let sum = tuple1.0 + tuple1.1 + tuple1.2 + tuple1.3
                    + tuple2.0 + tuple2.1 + tuple2.2 + tuple2.3
                    + tuple3.0 + tuple3.1 + tuple3.2 + tuple3.3
                    + tuple4.0 + tuple4.1 + tuple4.2 + tuple4.3
                    + tuple5.0 + tuple5.1 + tuple5.2 + tuple5.3
                    + tuple6.0 + tuple6.1 + tuple6.2;
            assert_eq(sum + a + b, 576u32);
        };
        return f;
    }
}

