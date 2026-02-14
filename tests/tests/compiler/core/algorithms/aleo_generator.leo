
program test.aleo {
    transition test_generator() -> group {
        let g: group = Aleo::generator();
        return g;
    }

    transition test_generator_powers() -> [group; 251u32] {
        let powers: [group; 251u32] = Aleo::generator_powers();
        return powers;
    }

    transition test_generator_powers_indexed() -> group {
        let g0: group = Aleo::generator_powers()[0u32];
        return g0;
    }

    async transition test_in_finalize() -> Future {
        return finalize_test();
    }

    async function finalize_test() {
        let g: group = Aleo::generator();
        let g0: group = Aleo::generator_powers()[0u32];
        assert_eq(g, g0);
    }
}
