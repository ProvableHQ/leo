struct Foo {
    x: u8?,
}

struct Wrapper {
    arr: [Foo?; 2],
}

struct Nested {
    wrapper: Wrapper,
}

program test.aleo {
    fn main() -> Wrapper {
        return Wrapper {
            arr: [Foo { x: 8u8 }, none] // implicit wrap
        };
    }

    fn nested() -> Nested {
        return Nested {
            wrapper: Wrapper {
                arr: [Foo { x: 9u8 }, none]
            }
        };
    }
}
