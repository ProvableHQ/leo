program ntzseals.aleo {
    // The 'nztseals' main function.
    // From Hacker's Delight 2nd ed. figure 5-25
    transition main(public x: u32) -> u8 {
        // 0x0450FBAF = 72416175
        x = (x & 0u32.sub_wrapped(x)).mul_wrapped(72416175u32);
        return sealsTableLookup(x >> 26u8);
    }

    // Right now we do not have any structure that allows
    // computable indexing, so simulate that with a function.
    // {32, 0, 1,12, 2, 6, u,13,   3, u, 7, u, u, u, u,14,
    //  10, 4, u, u, 8, u, u,25,   u, u, u, u, u,21,27,15,
    //  31,11, 5, u, u, u, u, u,   9, u, u,24, u, u,20,26,
    //  30, u, u, u, u,23, u,19,  29, u,22,18,28,17,16, u};

    function sealsTableLookup(i: u32) -> u8 {

        if i == 0u32 {return 32u8;} else
        if i == 1u32 {return 0u8;} else
        if i == 2u32 {return 1u8;} else
        if i == 3u32 {return 12u8;} else
        if i == 4u32 {return 2u8;} else
        if i == 5u32 {return 6u8;} else
        if i == 6u32 {return 0u8;} else  // unused
        if i == 7u32 {return 13u8;} else

        if i == 8u32 {return 3u8;} else
        if i == 9u32 {return 0u8;} else  // unused
        if i == 10u32 {return 7u8;} else
        if i == 11u32 {return 0u8;} else  // unused
        if i == 12u32 {return 0u8;} else  // unused
        if i == 13u32 {return 0u8;} else  // unused
        if i == 14u32 {return 0u8;} else  // unused
        if i == 15u32 {return 14u8;} else

        if i == 16u32 {return 10u8;} else
        if i == 17u32 {return 4u8;} else
        if i == 18u32 {return 0u8;} else  // unused
        if i == 19u32 {return 0u8;} else  // unused
        if i == 20u32 {return 8u8;} else
        if i == 21u32 {return 0u8;} else  // unused
        if i == 22u32 {return 0u8;} else  // unused
        if i == 23u32 {return 25u8;} else

        if i == 24u32 {return 0u8;} else  // unused
        if i == 25u32 {return 0u8;} else  // unused
        if i == 26u32 {return 0u8;} else  // unused
        if i == 27u32 {return 0u8;} else  // unused
        if i == 28u32 {return 0u8;} else  // unused
        if i == 29u32 {return 21u8;} else
        if i == 30u32 {return 27u8;} else
        if i == 31u32 {return 15u8;} else

        if i == 32u32 {return 31u8;} else
        if i == 33u32 {return 11u8;} else
        if i == 34u32 {return 5u8;} else
        if i == 35u32 {return 0u8;} else  // unused
        if i == 36u32 {return 0u8;} else  // unused
        if i == 37u32 {return 0u8;} else  // unused
        if i == 38u32 {return 0u8;} else  // unused
        if i == 39u32 {return 0u8;} else  // unused

        if i == 40u32 {return 9u8;} else
        if i == 41u32 {return 0u8;} else  // unused
        if i == 42u32 {return 0u8;} else  // unused
        if i == 43u32 {return 24u8;} else
        if i == 44u32 {return 0u8;} else  // unused
        if i == 45u32 {return 0u8;} else  // unused
        if i == 46u32 {return 20u8;} else
        if i == 47u32 {return 26u8;} else

        if i == 48u32 {return 30u8;} else
        if i == 49u32 {return 0u8;} else  // unused
        if i == 50u32 {return 0u8;} else  // unused
        if i == 51u32 {return 0u8;} else  // unused
        if i == 52u32 {return 0u8;} else  // unused
        if i == 53u32 {return 23u8;} else
        if i == 54u32 {return 0u8;} else  // unused
        if i == 55u32 {return 19u8;} else

        if i == 56u32 {return 29u8;} else
        if i == 57u32 {return 0u8;} else  // unused
        if i == 58u32 {return 22u8;} else
        if i == 59u32 {return 18u8;} else
        if i == 60u32 {return 28u8;} else
        if i == 61u32 {return 17u8;} else
        if i == 62u32 {return 16u8;} else
        if i == 63u32 {return 0u8;} else   // unused
        {return 0u8;} // unused
    }
}
