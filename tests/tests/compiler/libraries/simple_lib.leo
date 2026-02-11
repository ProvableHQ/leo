// --- Library: my_lib --- //

const X: u32 = 42;

fn boo() -> u32 {
    return 42u32 * 42;
}

fn foo() -> u32 {
    return boo() + X;
}

struct Foo {
    x: u32,
}

// --- Next Program --- //

import my_lib.aleo;

const X: u32 = 99;

struct Foo {
    x: u32,
}

program with_lib.aleo {
    @noupgrade
    constructor() {}

    fn main(i: u32, j: u32) -> (u32, Foo, my_lib.aleo/Foo) {
        let f1 = Foo { x: 42 };
        let f2 = my_lib.aleo/Foo { x: 99 };
        let sum = X + my_lib.aleo/X
            + bar(i)
            + my_lib.aleo/foo()
            + my_lib.aleo/boo()
            + f1.x
            + f2.x;
        return (sum, f1, f2);
    }
}

fn bar(i: u32) -> u32 {
    return boo(i);
}

fn boo(i: u32) -> u32 {
    return i * i;
}
