program child.aleo {
    fn ping() {}

    @custom
    constructor() {
        assert(std::ctx::edition() >= 0u16);
        assert_eq(std::ctx::checksum(), std::ctx::checksum());
        assert_eq(std::ctx::program_owner(), std::ctx::program_owner());
        assert_eq(std::ctx::addr(), std::ctx::addr());
    }
}
