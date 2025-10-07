program test.aleo {
    transition main() -> u32 {
        return 0u32; 
    }

    inline foo::[N: u32]() -> u32 {
        let N: u32 = 10u32;
        return N;
    }
}

