program control_flow_demo.aleo {
    fn if_else_demo() -> u8 {
        // ANCHOR: if_else
        let a: u8 = 1u8;

        if a == 1u8 {
            a += 1u8;
        } else if a == 2u8 {
            a += 2u8;
        } else {
            a += 3u8;
        }
        // ANCHOR_END: if_else
        return a;
    }

    fn ternary_demo() -> u8 {
        // ANCHOR: ternary
        let a: u8 = 1u8;
        a = (a == 1u8) ? a + 1u8 : ((a == 2u8) ? a + 2u8 : a + 3u8);
        // ANCHOR_END: ternary
        return a;
    }

    fn return_demo() -> u8 {
        // ANCHOR: return_stmt
        let a: u8 = 1u8;

        if a == 1u8 {
            return a + 1u8;
        } else if a == 2u8 {
            return a + 2u8;
        } else {
            return a + 3u8;
        }
        // ANCHOR_END: return_stmt
    }

    fn for_loop_demo() -> u32 {
        // ANCHOR: for_loop
        let count: u32 = 0u32;

        for i: u32 in 0u32..5u32 {
            count += 1u32;
        }

        return count; // returns 5u32
        // ANCHOR_END: for_loop
    }

    @noupgrade
    constructor() {}
}
