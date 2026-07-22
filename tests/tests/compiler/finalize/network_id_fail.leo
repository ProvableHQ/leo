program test.aleo {
    fn matches(nid: u16) {
        assert_eq(nid, std::ctx::network_id());
        return;
    }

    @noupgrade
    constructor() {}
}
