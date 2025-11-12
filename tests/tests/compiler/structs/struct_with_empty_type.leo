program scratch.aleo {
    struct Baz {
        a: u32,
        b: [u32; 0],
    }

    mapping baz: Baz => u64;

    async transition prepare(a: [u32; 0]) ->  Future {
        return finalize(a);
    }

    async function finalize(a: [u32;0]) {
        Mapping::set(baz, Baz { a: 0, b: [] }, 3);
    }

}

