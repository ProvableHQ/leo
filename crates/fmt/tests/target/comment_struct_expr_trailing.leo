struct Foo {
    x: u8?,
}

struct Wrapper {
    arr: [Foo?; 2],
}

struct Nested {
    wrapper: Wrapper,
}

fn build_wrapper() -> Wrapper {
    return Wrapper {
        arr: [Foo { x: 8u8 }, none], // implicit wrap
    };
}

fn build_nested() -> Nested {
    return Nested {
        wrapper: Wrapper {
            arr: [Foo { x: 9u8 }, none],
        },
    };
}

program test.aleo {
    fn main() -> [u8; 2] {
        return [
            build_wrapper().arr[0u32].unwrap().x.unwrap(),
            build_nested().wrapper.arr[0u32].unwrap().x.unwrap(),
        ];
    }
}
