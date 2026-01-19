fn foo1::[N: u32](x: u32) -> u32 {
    return N;
}

async fn foo2::[N: u32](x: u32) { }


script baz::[N: u32]()  { }

program const_generics.aleo {
    async fn main() -> Future {
        return foo2::[1u32](0u32);
    }
    fn bar1::[N: u32]() { }

    async fn bar2::[N: u32]() -> Future {
        return foo2::[1u32](0u32);
    }
}
