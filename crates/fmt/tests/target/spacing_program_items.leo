program test.aleo {
    struct A {
        x: u64,
    }

    struct B {
        y: u64,
    }

    fn foo(a: u64) -> u64 {
        return a;
    }

    fn bar(b: u64) -> u64 {
        return b;
    }

    const VALUE: u64 = 42u64;
    mapping store: u64 => u64;

    fn baz(c: u64) -> u64 {
        return c;
    }
}
