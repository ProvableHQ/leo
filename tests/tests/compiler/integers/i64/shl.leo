
program test.aleo {
    fn main(a: i64, b: i64, c: i64) -> bool {
        return a << 2u8 == a << 2u16 && a << 2u32 == c;
    }
}
