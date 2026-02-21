
program test.aleo {
    fn main(a: u64, b: u64, c: u64) -> bool {
        return a ** 2u8 == a ** 2u16 && a ** 2u32 == c;
    }
}
