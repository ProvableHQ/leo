program test.aleo {
    inline helper(x: u32) -> u32 {
        return x + 1u32;
    }

    inline wrapper(y: u32) -> u32 {
        return helper(y) * 2u32;
    }

    transition main(input: u32) -> u32 {
        return wrapper(input);
    }
}
