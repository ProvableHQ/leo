program test.aleo {
    fn test_generator() -> group {
        let g: group = Aleo::generator();
        return g;
    }

    fn test_generator_powers() -> [group; 251u32] {
        let powers: [group; 251u32] = Aleo::generator_powers();
        return powers;
    }

    fn test_generator_powers_indexed() -> group {
        let g0: group = Aleo::generator_powers()[0u32];
        return g0;
    }

    fn test_in_finalize() -> Final {
        return final { finalize_test(); };
    }
}

final fn finalize_test() {
    let g: group = Aleo::generator();
    let g0: group = Aleo::generator_powers()[0u32];
    assert_eq(g, g0);
}
