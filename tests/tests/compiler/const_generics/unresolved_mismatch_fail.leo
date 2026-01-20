struct Foo::[N: u32] { x: u32 }

program foo.aleo {
    fn main() {
        let f = foo::[0]();
    }
}

// Here, `Foo::[2 * N]` can be resolved but `Foo::[2 * N * x]` can't.
// Ensure we emit a monomorphization error (not a type check error).
fn foo::[N: u32]() -> Foo::[2 * N] {
    let x: u32 = 1u32;
    return Foo::[2 * N * x] { x: 0 };
}
