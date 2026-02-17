program test.aleo {
    mapping map: u32 => u32;
    fn main() -> (u32, Final) {
        let x = 5u32;
        let f = final {
            map.set(0u32, 42u32);
            assert(map.get(0u32) == 42u32);
            map.set(1u32, map.get(0u32) + x);
            assert(x == 5); // ensure we're using the old value of `x` not the updated one after the async block
            x = 6; // ensure we're able to update `x` in here
            assert(x == 6);
            assert(map.get(1u32) == 42u32 + x - 1);
        };
        assert(x == 5); // ensure `x` still has its old value
        x = 7;
        assert(x == 7);

        // Ensures that that the async block is actually executed (including the asserts).
        // This is technically illegal since `await` is not allowed in a transition. However, for testing purposes, this
        // seems fine for now. We should improve this when we have better testing infra for the interpreter
        f.run();

        assert(x == 7); // x should still have its value update outside the async block
        return (x, f);
    }
}
