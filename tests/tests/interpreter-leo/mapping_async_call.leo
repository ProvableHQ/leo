program test.aleo {
    mapping map: u32 => u32;
    fn main() -> (u32, Final) {
        let x = 5u32;
        let f = final { finalize(x); };
        x = 6;
        assert(x == 6);

        // Ensures that that the async call is actually executed (including the asserts in `finalize`).
        // This is technically illegal since `await` is not allowed in a transition. However, for testing purposes, this
        // seems fine for now. We should improve this when we have better testing infra for the interpreter
        f.run(); 
        return (x, f);
    }
}
final fn finalize(x: u32) {
    map.set(0u32, 42u32);
    assert(map.get(0u32) == 42u32);
    map.set(1u32, map.get(0u32) + x);
    assert(x == 5); // ensure we're using the old value of `x` not the updated one after the async block
    assert(map.get(1u32) == 42u32 + 5);
}
