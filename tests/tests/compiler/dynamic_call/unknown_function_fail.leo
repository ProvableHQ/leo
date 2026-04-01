interface Foo {
    fn bar() -> u32;
}

program test.aleo: Foo {
    fn bar() -> u32 {
        return 1u32;
    }

    fn main(target: field) -> u32 {
        return Foo@(target)::baz(1u32);
    }
}
