struct Baz {
    a: u32,
    b: [u32; 0],
}

program scratch.aleo {
    mapping baz: Baz => u64;

    fn prepare(a: [u32; 0]) ->  Final {
        return final { finalize(a); };
    }
}

final fn finalize(a: [u32;0]) {
    Mapping::set(baz, Baz { a: 0, b: [] }, 3);
}

