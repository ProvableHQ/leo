struct Foo { // struct level note
    x: u8?, // optional field
    y: [u8; 2], // array field
}

program test.aleo {
    fn main() -> Foo {
        return Foo { x: none, y: [1u8, 2u8] };
    }
}
