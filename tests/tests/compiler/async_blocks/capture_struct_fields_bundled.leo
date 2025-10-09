program test.aleo {
    struct Data {
        amount: u32,
        count: u32,
        flag: bool,
    }

    // Test capturing individual struct fields (similar to tuple fields)
    // Each field access counts as a separate capture
    async transition main() -> Future {
        let d1 = Data { amount: 1u32, count: 10u32, flag: true };
        let d2 = Data { amount: 2u32, count: 20u32, flag: false };
        let d3 = Data { amount: 3u32, count: 30u32, flag: true };
        let d4 = Data { amount: 4u32, count: 40u32, flag: false };
        let d5 = Data { amount: 5u32, count: 50u32, flag: true };
        let d6 = Data { amount: 6u32, count: 60u32, flag: false };
        let d7 = Data { amount: 7u32, count: 70u32, flag: true };
        let d8 = Data { amount: 8u32, count: 80u32, flag: false };
        let d9 = Data { amount: 9u32, count: 90u32, flag: true };
        let d10 = Data { amount: 10u32, count: 100u32, flag: false };
        let d11 = Data { amount: 11u32, count: 110u32, flag: true };
        
        let s0 = 0u32;
        let s1 = 1u32;
        let s2 = 2u32;
        let s3 = 3u32;
        let s4 = 4u32;
        let s5 = 5u32;

        // Total: 11 structs + 6 scalars = 17 captures
        // Should trigger bundling of 1 variable
        let f = async {
            let sum_amounts = d1.amount + d2.amount + d3.amount + d4.amount + d5.amount + d6.amount
                            + d7.amount + d8.amount + d9.amount + d10.amount + d11.amount;
            let sum_counts = d1.count + d2.count + d3.count + d4.count + d5.count + d6.count
                           + d7.count + d8.count + d9.count + d10.count + d11.count;
            let all_flags = d1.flag && !d2.flag && d3.flag && !d4.flag && d5.flag && !d6.flag
                          && d7.flag && !d8.flag && d9.flag && !d10.flag && d11.flag;
            let scalar_sum = s0 + s1 + s2 + s3 + s4 + s5;
            
            assert_eq(sum_amounts, 66u32);
            assert_eq(sum_counts, 660u32);
            assert(all_flags);
            assert_eq(scalar_sum, 15u32);
        };
        return f;
    }
}

