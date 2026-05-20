// Companion to #29428: mutating storage Vector/Mapping ops on a non-path
// receiver (e.g. a ternary over local storage) used to be reported as
// "on external vectors/mappings", which is misleading when the receiver
// is local. Now reported with the dedicated non-path diagnostic.
program bugpoc_write.aleo {
    @noupgrade
    constructor() {}

    storage h1: [u64];
    storage h2: [u64];
    mapping m1: u32 => u32;
    mapping m2: u32 => u32;

    fn vec_set(c: bool) -> Final {
        return final { (c ? h1 : h2).set(0u32, 1u64); }; // ❌ non-path receiver
    }

    fn vec_push(c: bool) -> Final {
        return final { (c ? h1 : h2).push(1u64); }; // ❌ non-path receiver
    }

    fn vec_pop(c: bool) -> Final {
        return final {
            let x: u64? = (c ? h1 : h2).pop(); // ❌ non-path receiver
            assert(x != none);
        };
    }

    fn vec_swap_remove(c: bool) -> Final {
        return final {
            let x: u64 = (c ? h1 : h2).swap_remove(0u32); // ❌ non-path receiver
            assert(x >= 0u64);
        };
    }

    fn vec_clear(c: bool) -> Final {
        return final { (c ? h1 : h2).clear(); }; // ❌ non-path receiver
    }

    fn map_set(c: bool) -> Final {
        return final { (c ? m1 : m2).set(0u32, 1u32); }; // ❌ non-path receiver
    }

    fn map_remove(c: bool) -> Final {
        return final { (c ? m1 : m2).remove(0u32); }; // ❌ non-path receiver
    }
}
