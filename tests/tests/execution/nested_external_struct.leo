/*
seed = 123456789
min_height = 16

[case]
program = "external_structs.aleo"
function = "main"
input = []
*/

struct Foo {
    c: Boo,
}
struct Boo {
    a: u32,
    b: u32,
}
struct Coo { // Coo is only used in `external_structs.aleo`. Make sure it doesn't get removed from `child1`.
    x: u32,
    y: u32,
}
program child1.aleo {
    fn main_2(a: Foo) -> Boo { 
      return a.c;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import child1.aleo;
struct Foo {
    c: child1.aleo/Foo // ensures we don't think this is a cyclical dependency
}
program external_structs.aleo {
    fn main() -> (child1.aleo/Boo, child1.aleo/Foo, child1.aleo/Boo, Foo, child1.aleo/Coo) {
        let bb: child1.aleo/Boo = child1.aleo/Boo {a:1u32, b:1u32};
        let ff = child1.aleo/Foo {c: bb};

        return (bb, ff, child1.aleo/main_2(ff), Foo {c: ff}, child1.aleo/Coo { x: 0, y: 0});
    }

    @noupgrade
    constructor() {}
}
