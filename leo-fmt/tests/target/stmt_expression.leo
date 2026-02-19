program test.aleo {
    function process(a: u64, b: u64, c: u64) {
        assert(a > 0u64);
    }

    transition main(item: u64) {
        process(item, item, item);
        process(item, 1u64, 2u64);
    }
}
