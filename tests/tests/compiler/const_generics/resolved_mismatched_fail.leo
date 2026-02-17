struct Foo::[N: u32] { x: u32 }

program foo.aleo {
    fn main() {
        let f = foo::[0]();
    }
}

// Here, both `Foo::[2 * N]` and `Foo::[2 * N + 1]` can be resolved.
// Ensure we emit a type check error.
fn foo::[N: u32]() -> Foo::[2 * N] {
    let x: u32 = 1u32;
    return Foo::[2 * N + 1] { x: 0 };
}
