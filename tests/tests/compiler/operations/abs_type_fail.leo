
program test.aleo {
    transition main() -> u8 {
        return (true ? 1u16.abs() : 2u16.abs()) as u8;
    }
}
