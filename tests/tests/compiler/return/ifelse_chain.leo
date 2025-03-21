
// The intent is to test that the amount of code generated
// is only linear in the number of conditions.
program test.aleo {
    transition main(a: field) -> u32 {
        if a == 0field {
            return 0u32;
        } else if a == 1field {
            return 1u32;
        } else if a == 2field {
            return 2u32;
        } else if a == 3field {
            return 3u32;
        } else if a == 4field {
            return 4u32;
        }

        return 5u32;
    }
}
