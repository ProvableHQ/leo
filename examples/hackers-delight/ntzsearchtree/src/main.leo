program ntzsearchtree.aleo {
    // The 'ntzsearchtree' main function.
    // From Hacker's Delight 2nd ed. figure 5-22,
    // expanded to a 32-bit version.
    transition main(public x: u32) -> u8 {
        if (x & 65535u32 != 0u32) {
            if (x & 255u32 != 0u32) {
                if (x & 15u32 != 0u32) {
                    if (x & 3u32 != 0u32) {
                        if (x & 1u32 != 0u32) {return 0u8;}
                        else {return 1u8;} }
                    else {
                        if (x & 4u32 != 0u32) {return 2u8;}
                        else {return 3u8;} } }
                else { // low 4 bits are 0 but low 8 bits are not zero
                    if (x & 48u32 != 0u32) {  // 48 = 0011 0000
                        if (x & 16u32 != 0u32) {return 4u8;}
                        else {return 5u8;} }
                    else {if (x & 64u32 != 0u32) {return 6u8;}
                        else {return 7u8;} } } }
            else { // low 8 bits are 0 but low 16 bits are not zero
                // 3840 = 00001111 00000000
                if (x & 3840u32 != 0u32) {
                    // 768 = 00000011 00000000
                    if (x & 768u32 != 0u32) {
                        if (x & 256u32 != 0u32) {return 8u8;}
                        else {return 9u8;} }
                    else { // 1024 = 00000100 00000000
                        if (x & 1024u32 != 0u32) {return 10u8;}
                        else {return 11u8;} } }
                else { // low 12 bits are 0 but low 16 bits are not zero
                    // 12288 = 0011 0000 0000 0000
                    if (x & 12288u32 != 0u32) {
                        // 4096 = 0001 0000 0000 0000
                        if (x & 4096u32 != 0u32) {return 12u8;}
                        else {return 13u8;} }
                    else { // low 14 bits are 0 but low 16 bits are not zero
                        // 16384 = 0100 0000 0000 0000
                        if (x & 16384u32 != 0u32) {return 14u8;}
                        else {return 15u8;} } } } }
        else { // low 16 bits are zero, now look at high 16 bits
            // Simply by multiplying the previous constants by 65536
            if (x & (255u32 * 65536u32) != 0u32) {
                if (x & (15u32 * 65536u32) != 0u32) {
                    if (x & (3u32 * 65536u32) != 0u32) {
                        if (x & (1u32 * 65536u32) != 0u32) {return 16u8;}
                        else {return 17u8;} }
                    else {
                        if (x & (4u32 * 65536u32) != 0u32) {return 18u8;}
                        else {return 19u8;} } }
                else {
                    if (x & (48u32 * 65536u32) != 0u32) {
                        if (x & (16u32 * 65536u32) != 0u32) {return 20u8;}
                        else {return 21u8;} }
                    else {
                        if (x & (64u32 * 65536u32) != 0u32) {return 22u8;}
                        else {return 23u8;} } } }
            else {
                if (x & (3840u32 * 65536u32) != 0u32) {
                    if (x & (768u32 * 65536u32) != 0u32) {
                       if (x & (256u32 * 65536u32) != 0u32) {return 24u8;}
                       else {return 25u8;} }
                    else {
                        if (x & (1024u32 * 65536u32) != 0u32) {return 26u8;}
                        else {return 27u8;} } }
                else {
                    if (x & (12288u32 * 65536u32) != 0u32) {
                        if (x & (4096u32 * 65536u32) != 0u32) {return 28u8;}
                        else {return 29u8;} }
                    else {
                        if (x & (16384u32 * 65536u32) != 0u32) {return 30u8;}
                        else {
                            if (x != 0u32) {return 31u8;}
                            else {return 32u8;} } } } } }
    }
}
