program test.aleo {
    mapping foo: u32 => u32;

    fn t1() -> Final {
        return final {
            let b = std::ctx::block_height();
            let t = std::ctx::block_timestamp();
            let i = std::ctx::network_id();
            foo.set(0u32, 0u32);
            let s1 = foo.get(0u32);
            let s2 = foo.get_or_use(0u32, 0u32);
            foo.remove(0u32);
            assert(!foo.contains(0u32));
        };
    }

    @noupgrade
    constructor() {}
}
