// Test: Interface extending multiple interfaces

interface A {
    fn foo() -> u64;
}

interface B {
    fn bar() -> u64;
}

interface C : A + B {
    fn baz() -> u64;
}

program test.aleo : C {
    fn foo() -> u64 {
        return 1u64;
    }

    fn bar() -> u64 {
        return 2u64;
    }

    fn baz() -> u64 {
        return 3u64;
    }

    fn main(x: u64) -> u64 {
        return 0;
    }
}
