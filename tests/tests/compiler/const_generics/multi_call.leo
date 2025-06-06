program test.aleo {
    transition main() -> u32 {
        return bar::[5u32, 1u32]() + bar::[6u32, 2u32]();
    }

    inline bar::[N: u32, M: u32]() -> u32 {
        return N + M + foo1() + foo2();
    }

    inline foo1() -> u32 { return 1000u32; }
    inline foo2() -> u32 { return 10u32; }
}
