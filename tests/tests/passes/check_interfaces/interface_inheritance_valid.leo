// Test: Valid interface inheritance with compatible members

interface Base {
    fn get_value() -> u64;
}

interface Extended : Base {
    fn set_value(v: u64) -> u64;
}

program test.aleo : Extended {
    fn get_value() -> u64 {
        return 0u64;
    }

    fn set_value(v: u64) -> u64 {
        return v;
    }

    fn main(x: u64) -> u64 {
        return set_value(x);
    }
}
