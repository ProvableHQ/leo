program test.aleo {
    fn bubble(a: u32, b: u32) -> u32 {
        // compare first pair
        if a < b {
            return a;
        }
        // compare second pair
        if b < a {
            return b;
        }
        return a;
    }
}
