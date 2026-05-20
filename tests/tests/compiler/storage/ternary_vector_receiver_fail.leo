// Regression test for https://github.com/ProvableHQ/leo/issues/29428
// `Vector::len` / `Vector::get` on a non-path receiver (e.g. a ternary over
// storage vectors) used to ICE in storage_lowering. Now caught cleanly by
// type-checking.
program bugpoc.aleo {
    @noupgrade
    constructor() {}

    storage h1: [u64];
    storage h2: [u64];
    mapping seen: u32 => u32;

    fn method_len(c: bool) -> Final {
        return final {
            let n: u32 = (c ? h1 : h2).len(); // ❌ non-path receiver
            Mapping::set(seen, 0u32, n);
        };
    }

    fn method_get(c: bool) -> Final {
        return final {
            let v: u64? = (c ? h1 : h2).get(0u32); // ❌ non-path receiver
            Mapping::set(seen, 0u32, v.unwrap_or(0u64) as u32);
        };
    }

    fn explicit_len(c: bool) -> Final {
        return final {
            let n: u32 = Vector::len((c ? h1 : h2)); // ❌ non-path receiver
            Mapping::set(seen, 0u32, n);
        };
    }
}
