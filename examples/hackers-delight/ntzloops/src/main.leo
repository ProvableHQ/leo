program ntzloops.aleo {
    // The 'ntzloops' main function.
    // From Hacker's Delight 2nd ed. figure 5-23
    transition main(public x: u32) -> u8 {
        x = !x & x.sub_wrapped(1u32);
        let n: u8 = 0u8;
        for i:u8 in 0u8..32u8 {
            if x != 0u32 {
                n += 1u8;
                x = x >> 1u8;
            }
        }
        return n;
    }
}
