program test.aleo {
    fn main(x: u32, condition: bool) -> u32 {
        let result: u32 = 0u32;
        if condition {
            result = x + 1u32;
        } else {
            result = x + 2u32;
        }
        return result;
    }
}
