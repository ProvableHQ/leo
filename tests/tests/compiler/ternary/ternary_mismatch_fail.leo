
program test.aleo {
    fn main() -> u32 {
        return (true ? 1u8 : 1u16) as u32;
    }
}
