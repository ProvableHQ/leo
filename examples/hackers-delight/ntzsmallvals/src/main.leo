program ntzsmallvals.aleo {
    // The 'ntzsmallvals' main function.
    // From Hacker's Delight 2nd ed. figure 5-21
    transition main(public x: u32) -> u8 {
        if (x == 0u32) {return 32u8;}
        let n: u8 = 31u8;
        let y: u32 = x.shl_wrapped(16u8); if (y != 0u32) {n = n - 16u8; x = y;}
        y = x.shl_wrapped(8u8); if (y != 0u32) {n = n - 8u8; x = y;}
        y = x.shl_wrapped(4u8); if (y != 0u32) {n = n - 4u8; x = y;}
        y = x.shl_wrapped(2u8); if (y != 0u32) {n = n - 2u8; x = y;}
        y = x.shl_wrapped(1u8); if (y != 0u32) {n = n - 1u8;}
        return n;
    }
}
