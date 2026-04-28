interface Base {
    fn fetch_value() -> u64;
}

interface Extended : Base {
    fn store_value(v: u64);
}

program bar.aleo : Extended {
    fn fetch_value() -> u64 {
        return 0u64;
    }

    fn store_value(v: u64) {
        assert_eq(v, v);
    }

    @noupgrade
    constructor() {}
}
