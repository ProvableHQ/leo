program test.aleo {
    fn matches(addr: address) -> (bool, Final) {
        return (
            std::ctx::caller() == addr,
            final {
                finalize_matches(std::ctx::caller());
            },
        );
    }

    @noupgrade
    constructor() {}
}

final fn finalize_matches(addr: address) {
    assert_eq(addr, std::ctx::caller());
}
