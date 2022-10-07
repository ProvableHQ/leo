program ntzdebruijn.aleo {
    // The 'ntzdebruijn' main function.
    // From Hacker's Delight 2nd ed. figure 5-26
    transition main(public x: u32) -> u8 {
        if x == 0u32 {return 32u8;}
        // 0x04D7651F = 81224991
        x = (x & 0u32.sub_wrapped(x)).mul_wrapped(81224991u32);
        let i: u32 = x >> 27u8;
        return deBruijnTableLookup(i);
    }

    //     { 0, 1, 2,24, 3,19, 6,25,   22, 4,20,10,16, 7,12,26,
    //      31,23,18, 5,21, 9,15,11,   30,17, 8,14,29,13,28,27};

    function deBruijnTableLookup(i: u32) -> u8 {

        if i == 0u32 {return 0u8;} else
        if i == 1u32 {return 1u8;} else
        if i == 2u32 {return 2u8;} else
        if i == 3u32 {return 24u8;} else
        if i == 4u32 {return 3u8;} else
        if i == 5u32 {return 19u8;} else
        if i == 6u32 {return 6u8;} else
        if i == 7u32 {return 25u8;} else

        if i == 8u32 {return 22u8;} else
        if i == 9u32 {return 4u8;} else
        if i == 10u32 {return 20u8;} else
        if i == 11u32 {return 10u8;} else
        if i == 12u32 {return 16u8;} else
        if i == 13u32 {return 7u8;} else
        if i == 14u32 {return 12u8;} else
        if i == 15u32 {return 26u8;} else

        if i == 16u32 {return 31u8;} else
        if i == 17u32 {return 23u8;} else
        if i == 18u32 {return 18u8;} else
        if i == 19u32 {return 5u8;} else
        if i == 20u32 {return 21u8;} else
        if i == 21u32 {return 9u8;} else
        if i == 22u32 {return 15u8;} else
        if i == 23u32 {return 11u8;} else

        if i == 24u32 {return 30u8;} else
        if i == 25u32 {return 17u8;} else
        if i == 26u32 {return 8u8;} else
        if i == 27u32 {return 14u8;} else
        if i == 28u32 {return 29u8;} else
        if i == 29u32 {return 13u8;} else
        if i == 30u32 {return 28u8;} else
        if i == 31u32 {return 27u8;} else
        {return 0u8;} // unused
    }
}
