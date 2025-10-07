program test.aleo {
    transition main() -> u32 {
        return 0u32; 
    }

    inline foo::[N: u32](N: u32) -> u32 {
        return N;
    }
}

