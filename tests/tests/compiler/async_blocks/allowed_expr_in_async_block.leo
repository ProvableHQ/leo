program test.aleo {
    mapping foo: u32 => u32;

    fn t1() -> Final {
        return final {
            let _b = block.height;
            let _t = block.timestamp;
            let _i = network.id;
            foo.set(0u32, 0u32);
            let _s1 = foo.get(0u32);
            let _s2 = foo.get_or_use(0u32, 0u32);
            foo.remove(0u32);
            assert(!foo.contains(0u32));
        };
    }

    @noupgrade
    constructor() {}
}
