
program test.aleo {
    fn main() -> u32 {
        const a: (u8, u16, u32) = (1u8, 2u16, 3u32);
        return a.2;
    }
}
