program ntzreisers.aleo {
    // The 'ntzreisers' main function.
    // From Hacker's Delight 2nd ed. figure 5-27
    transition main(public x: u32) -> u8 {
        x = (x & 0u32.sub_wrapped(x)).rem_wrapped(37u32);
        return reisersTableLookup(x);
    }

    // There are 37 entries here
    // {32, 0, 1, 26, 2, 23, 27,
    //   u, 3, 16, 24, 30, 28, 11, u, 13, 4,
    //   7, 17, u, 25, 22, 31, 15, 29, 10, 12,
    //   6, u, 21, 14, 9, 5, 20, 8, 19, 18};

    function reisersTableLookup(i: u32) -> u8 {

        if i == 0u32 {return 32u8;} else
        if i == 1u32 {return 0u8;} else
        if i == 2u32 {return 1u8;} else
        if i == 3u32 {return 26u8;} else
        if i == 4u32 {return 2u8;} else
        if i == 5u32 {return 23u8;} else
        if i == 6u32 {return 27u8;} else

        if i == 7u32 {return 0u8;} else  // unused
        if i == 8u32 {return 3u8;} else
        if i == 9u32 {return 16u8;} else
        if i == 10u32 {return 24u8;} else
        if i == 11u32 {return 30u8;} else
        if i == 12u32 {return 28u8;} else
        if i == 13u32 {return 11u8;} else
        if i == 14u32 {return 0u8;} else // unused
        if i == 15u32 {return 13u8;} else
        if i == 16u32 {return 4u8;} else

        if i == 17u32 {return 7u8;} else
        if i == 18u32 {return 17u8;} else
        if i == 19u32 {return 0u8;} else // unused
        if i == 20u32 {return 25u8;} else
        if i == 21u32 {return 22u8;} else
        if i == 22u32 {return 31u8;} else
        if i == 23u32 {return 15u8;} else
        if i == 24u32 {return 29u8;} else
        if i == 25u32 {return 10u8;} else
        if i == 26u32 {return 12u8;} else

        if i == 27u32 {return 6u8;} else
        if i == 28u32 {return 0u8;} else // unused
        if i == 29u32 {return 21u8;} else
        if i == 30u32 {return 14u8;} else
        if i == 31u32 {return 9u8;} else
        if i == 32u32 {return 5u8;} else
        if i == 33u32 {return 20u8;} else
        if i == 34u32 {return 8u8;} else
        if i == 35u32 {return 19u8;} else
        if i == 36u32 {return 18u8;} else

        {return 0u8;} // unused
    }
}
