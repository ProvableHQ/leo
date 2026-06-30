program test.aleo {
    fn main(a: u16) -> Final {
        return final { finalize_main(a); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_main(a: u16) {
    assert_eq(1u16, std::ctx::network_id());
    assert_eq(a, std::ctx::network_id());
    assert_eq(std::ctx::network_id(), std::ctx::network_id());
}
