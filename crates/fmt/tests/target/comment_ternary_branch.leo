struct Foo {
    x: u8?,
}

struct Wrapper {
    arr: [Foo?; 2],
}

program test.aleo {
    fn main(cond: bool) -> Wrapper? {
        let result: Wrapper? = cond
            ? Wrapper { arr: [Foo { x: 99u8 }, none] } // implicit wrap
            : none;
        return result;
    }
}
