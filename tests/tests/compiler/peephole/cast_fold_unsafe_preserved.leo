program test.aleo {
    fn main(x: u64) -> u64 {
        let narrow: u16 = x as u16;
        let wide: u64 = narrow as u64;
        return wide;
    }
}
