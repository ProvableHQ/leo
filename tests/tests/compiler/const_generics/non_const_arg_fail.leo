program test.aleo {
    transition main(x: u32) -> u32 {
        return bar::[x]();
    }

    inline bar::[N: u32]() -> u32 {
        return N;
    }
}
