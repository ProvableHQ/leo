// Companion to #29428: ensure non-path receivers for `Mapping::get` /
// `Mapping::contains` / `Mapping::get_or_use` are also caught cleanly.
program bugpoc_map.aleo {
    @noupgrade
    constructor() {}

    mapping m1: u32 => u32;
    mapping m2: u32 => u32;

    fn method_get(c: bool) -> Final {
        return final {
            let v: u32 = (c ? m1 : m2).get(0u32); // ❌ non-path receiver
            assert(v >= 0u32);
        };
    }

    fn method_contains(c: bool) -> Final {
        return final {
            let b: bool = (c ? m1 : m2).contains(0u32); // ❌ non-path receiver
            assert(b == b);
        };
    }

    fn method_get_or_use(c: bool) -> Final {
        return final {
            let v: u32 = (c ? m1 : m2).get_or_use(0u32, 0u32); // ❌ non-path receiver
            assert(v >= 0u32);
        };
    }
}
