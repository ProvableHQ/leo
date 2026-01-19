program test.aleo {
    async fn main(a: u16) -> Future {
        return finalize_main(a);
    }
}

async fn finalize_main(a: u16) {
    assert_eq(1u16, network.id);
    assert_eq(a, network.id);
    assert_eq(network.id, network.id);
}
