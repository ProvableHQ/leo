// Test: Diamond pattern with compatible signatures

interface Base {
    fn get_value() -> u64;
}

interface Left : Base {
    fn left_op() -> u64;
}

interface Right : Base {
    fn right_op() -> u64;
}

interface Diamond : Left + Right {}

program test.aleo : Diamond {
    fn get_value() -> u64 {
        return 0u64;
    }

    fn left_op() -> u64 {
        return 1u64;
    }

    fn right_op() -> u64 {
        return 2u64;
    }

    fn main(x: u64) -> u64 {
        return 0;
    }
}
