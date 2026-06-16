program main.aleo {
    fn exercise() {
        let rp1 = std::ctx::addr();
        assert_eq(rp1, rp1);
        let rp2 = std::ctx::caller();
        assert_eq(rp2, rp2);
        let rp3 = std::ctx::signer();
        assert_eq(rp3, rp3);
        return;
    }

    fn exercise_async() -> Final {
        return final {
            let rf1 = std::ctx::id();
            assert_eq(rf1, rf1);
            let rf2 = std::ctx::checksum();
            assert_eq(rf2, rf2);
            let rf3 = std::ctx::edition();
            assert_eq(rf3, rf3);
            let rf4 = std::ctx::program_owner();
            assert_eq(rf4, rf4);
            let rf5 = std::ctx::block_height();
            assert_eq(rf5, rf5);
            let rf6 = std::ctx::block_timestamp();
            assert_eq(rf6, rf6);
            let rf7 = std::ctx::network_id();
            assert_eq(rf7, rf7);
        };
    }

    @noupgrade
    constructor() {}
}
