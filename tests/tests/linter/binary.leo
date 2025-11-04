
program test.aleo {
    transition division_by_zero() -> u32 {
        let a: u32 = 5u32 / 0u32;
        return a;
    }

    transition multiplication_by_one() -> u32 {
        let a: u32 = 5u32 * 1u32;
        return a;
    }

    transition addition_by_zero() -> u32 {
        let a: u32 = 5u32 + 0u32;
        return a;
    }
    
    transition division_by_one() -> u32 {
        let a: u32 = 5u32 / 1u32;
        return a;
    }

    transition irrefutable_pattern() {
        let a: bool = true == true;
        let b: bool = true != false;
        let c: bool = 4u64 == 4u64;
        let d: bool = 4u32 != 3u32;

        assert(a);
        assert(c);
        assert(b);
        assert(d);

        return;
    }
}
