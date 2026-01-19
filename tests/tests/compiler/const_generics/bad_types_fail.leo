struct Foo::[N: (u32, u32), A: [u32; 3]] {
    x: u32,
}

program const_generics.aleo {
    fn main() -> u32 {
        return bar::[9u16, 0u32]();
    }
}

@inline
fn bar::[N: u32, S: scalar]() -> u32 {
    return N;
}

@inline
fn foo::[N: (u32, u32), A: [u32; 3]]() -> (u32, u32) {
    return N;
}
