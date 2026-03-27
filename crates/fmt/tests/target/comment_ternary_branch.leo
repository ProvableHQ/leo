struct Foo {
    x: u8?,
}

struct Wrapper {
    arr: [Foo?; 2],
}

fn choose_wrapper(cond: bool) -> Wrapper? {
    let result: Wrapper? = cond
        ? Wrapper {
            arr: [Foo { x: 99u8 }, none],
        } // implicit wrap
        : none;
    return result;
}

program test.aleo {
    fn main(cond: bool) -> u8 {
        return choose_wrapper(cond).unwrap().arr[0u32].unwrap().x.unwrap();
    }
}
