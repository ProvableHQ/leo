program test.aleo {
    const VALUE: u32 = 10;

    transition main() -> u32 {
        const VALUE: u32 = 20;
        return VALUE;
    }
}

