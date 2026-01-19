struct Baz {
    a: u32,
    b: [u32; 0],
}

program scratch.aleo {
    mapping baz: Baz => u64;

    async fn prepare(a: [u32; 0]) ->  Future {
        return finalize(a);
    }
}

async fn finalize(a: [u32;0]) {
    Mapping::set(baz, Baz { a: 0, b: [] }, 3);
}

