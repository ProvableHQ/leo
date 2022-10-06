program ntzmasks.aleo {
    // The 'ntzmasks' main function.
    // From Hacker's Delight 2nd ed. figure 5-20
    transition main(public x: u32) -> u8 {
        if (x == 0u32) {return 32u8;}
        let n: u8 = 1u8;
        // x >>= 16u8 wasn't working, and I don't want to use
        // to u32 as a shift operand, so I do x = x >> 16u8
        if ((x & 65535u32) == 0u32) {n += 16u8; x = x >> 16u8;}
        if ((x & 255u32) == 0u32) {n += 8u8; x = x >> 8u8;}
        if ((x & 15u32) == 0u32) {n += 4u8; x = x >> 4u8;}
        if ((x & 3u32) == 0u32) {n += 2u8; x = x >> 2u8;}
        // can't do `return n - (x & 1u32);` because no typecasts, so:
        if ((x & 1u32) == 1u32) {n -= 1u8;}
        return n;
    }
}
