program const_generics.aleo {
    struct Foo::[N: (u32, u32), A: [u32; 3]] {
        x: u32,
    }

    transition main() -> u32 {
        return bar::[9u16, 0u32]();
    }

    inline bar::[N: u32, S: scalar]() -> u32 {
        return N;
    }

    inline foo::[N: (u32, u32), A: [u32; 3]]() -> (u32, u32) {
        return N;
    }
}
