program test.aleo {
    struct Flag {
        active: bool,
        payload: u32,
    }

    // End-to-end demonstration of the passes that #29343 adds together:
    // composite forwarding replaces `f.active` with the stored literal `true`,
    // then constant folding collapses the rest of the expression.
    fn main(x: u32, y: u32) -> u32 {
        let f: Flag = Flag { active: true, payload: x };
        return f.active ? f.payload : y;
    }
}
