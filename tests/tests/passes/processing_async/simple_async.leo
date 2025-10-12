program test.aleo {
    async transition main(x: u32) -> Future {
        return finalize_main(x);
    }

    async function finalize_main(x: u32) {
        assert_eq(x, 10u32);
    }
}
