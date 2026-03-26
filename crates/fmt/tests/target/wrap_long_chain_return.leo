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

fn unwrap_value(top: Top) -> u8 {
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

program test.aleo {
    fn main() -> u8 {
        let top: Top = Top {
            container: Container {
                wrappers: [
                    Wrapper {
                        arr: [Inner { val: 7u8 }, none],
                    },
                    none,
                ],
            },
        };
        return unwrap_value(top);
    }
}
