program test.aleo {
    fn matches(height: u32) -> Final {
        return final { finalize_matches(height); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_matches(height: u32) {
    assert_eq(height, std::ctx::block_height());
}
