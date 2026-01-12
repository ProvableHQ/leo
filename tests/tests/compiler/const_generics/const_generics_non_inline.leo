@no_inline
fn foo1::[N: u32](x: u32) -> u32 {
    return N;
}

final fn foo2::[N: u32](x: u32) { }


script baz::[N: u32]()  { }

program const_generics.aleo {
    fn main() -> Final {
        return final { foo2::[1u32](0u32); };
    }
    fn bar1::[N: u32]() { }

    fn bar2::[N: u32]() -> Final {
        return final { foo2::[1u32](0u32); };
    }
}
