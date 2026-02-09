program test.aleo {
    function compute(a: u64, b: u64) -> u64 {
        let x: u64 = a + b; // compute sum
        let y: u64 = x * 2u64; // double it
        assert(y > 0u64); // must be positive
        return y; // return result
    }

    transition check(a: u64, b: u64) -> bool {
        let result: bool = a >= b; // compare values
        assert_eq(result, true); // verify
        return result; // done
    }
}
