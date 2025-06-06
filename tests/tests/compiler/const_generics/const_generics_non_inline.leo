program const_generics.aleo {
    async transition main() -> Future {
        return foo2::[1u32](0u32);
    }

    function foo1::[N: u32](x: u32) -> u32 {
        return N;
    }

    async function foo2::[N: u32](x: u32) { }

    transition bar1::[N: u32]() { }

    async transition bar2::[N: u32]() -> Future {
        return foo2::[1u32](0u32);
    }

    script baz::[N: u32]()  { }
}
