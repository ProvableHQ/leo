const VALUE: u32 = 10;

program test.aleo {
    transition main() -> u32 {
        const VALUE: u32 = 20;
        return VALUE;
    }
}

