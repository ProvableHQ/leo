program ntzgaudet.aleo {
    // The 'ntzgaudet' main function.
    // From Hacker's Delight 2nd ed. figure 5-24
    transition main(public x: u32) -> u8 {
        let y: u32 = x & 0u32.sub_wrapped(x);  // Isolate rightmost 1-bit
        let bz: u8 = (y != 0u32) ? 0u8 : 1u8;
        // 0x0000FFFF = 65535
        let b4: u8 = (y & 65535u32 != 0u32) ? 0u8 : 16u8;
        // 0x00FF00FF = 16711935
        let b3: u8 = (y & 16711935u32 != 0u32) ? 0u8 : 8u8;
        // 0x0F0F0F0F = 252645135
        let b2: u8 = (y & 252645135u32 != 0u32) ? 0u8 : 4u8;
        // 0x33333333 = 858993459
        let b1: u8 = (y & 858993459u32 != 0u32) ? 0u8 : 2u8;
        // 0x55555555 = 1431655765
        let b0: u8 = (y & 1431655765u32 != 0u32) ? 0u8 : 1u8;
        return bz + b4 + b3 + b2 + b1 + b0;
    }
}
