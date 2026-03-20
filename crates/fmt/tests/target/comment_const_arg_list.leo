struct Foo::[X: u32, Y: u32] {
    value: u32,
}

program test.aleo {
    fn foo::[A: u32, B: u32, C: u32]() -> u32 {
        return A + B + C;
    }

    fn main() -> u32 {
        return foo::[
            1u32, // first
            2u32, // second
            3u32 // third
        ]();
    }

    fn make() -> Foo::[1u32, 2u32] {
        return Foo::[
            1u32, // x
            2u32 // y
        ] { value: 0u32 };
    }
}
