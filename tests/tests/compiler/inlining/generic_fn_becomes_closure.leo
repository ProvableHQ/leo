// Test: a monomorphized generic fn called more than once becomes a `closure` in
// Aleo bytecode, exactly like a non-generic fn.  The monomorphized name
// (`scale::[5u32]`) is hashed via `legalize_path` into a legal Aleo identifier
// at the declaration and at every call site, so both sides agree.

fn scale::[N: u32](x: u32) -> u32 {
    return x * N;
}

program test.aleo {
    fn main(x: u32, y: u32) -> (u32, u32) {
        // Same monomorphization called twice — emitted as a `closure`, not inlined.
        let a: u32 = scale::[5u32](x);
        let b: u32 = scale::[5u32](y);
        return (a, b);
    }
}
