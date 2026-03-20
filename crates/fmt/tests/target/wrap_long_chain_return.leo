struct Inner {
    val: u8?,
}

struct Wrapper {
    arr: [Inner?; 2]?,
}

struct Container {
    wrappers: [Wrapper?; 2]?,
}

struct Top {
    container: Container?,
}

program test.aleo {
    fn foo(top: Top) -> u8 {
        return top
            .container
            .unwrap()
            .wrappers
            .unwrap()[0]
            .unwrap()
            .arr
            .unwrap()[0]
            .unwrap()
            .val
            .unwrap();
    }
}
