/*
seed = 123456789
[case]
program = "parent.aleo"
function = "create_another_wrapper"
input = []
// TODO: enable this case when https://github.com/ProvableHQ/snarkVM/pull/3107 is released
// [case]
// program = "grandparent.aleo"
// function = "omega_wrapper"
// input = []
*/

struct Foo {
    bar: [Bar; 1],
}

struct Bar {
    baz: [Baz; 2],
}

struct Baz {
    one: One,
}

struct One {
    two: [Two; 2],
}

struct Two {
    val1: u32,
    val2: u32,
}

program child.aleo {
    record Boo {
        owner: address,
        val: u32,
    }

    fn create() -> (Foo, Boo) {
        return (Foo {bar: [Bar {baz: [Baz {one: One {two: [Two {val1: 1u32, val2: 2u32}, Two {val1: 3u32, val2:
        4u32}]}}, Baz {one: One {two: [Two {val1: 5u32, val2: 6u32}, Two {val1: 7u32, val2: 8u32}]}}]}]}, Boo {owner:
        self.signer, val: 10u32});
    }
}

// --- Next Program --- //

import child.aleo;

struct Woo {
    a: u32,
    b: u32,
}

program parent.aleo {

    record BooHoo {
        owner: address,
        val: u32,
        woo: Woo,
    }

    fn create_wrapper() -> (child.aleo/Foo, child.aleo/Boo) {
        let f: child.aleo/Foo = child.aleo/Foo {bar: [child.aleo/Bar {baz: [child.aleo/Baz {one: child.aleo/One {two: [child.aleo/Two {val1: 1u32, val2: 2u32}, child.aleo/Two {val1: 3u32, val2: 4u32}]}}, child.aleo/Baz {one: child.aleo/One {two: [child.aleo/Two {val1: 5u32, val2: 6u32}, child.aleo/Two {val1: 7u32, val2: 8u32}]}}]}]};
        let (f1, b1): (child.aleo/Foo, child.aleo/Boo) = child.aleo/create();
        return child.aleo/create();
    }

    fn create_another_wrapper() -> (child.aleo/Foo, child.aleo/Boo, BooHoo, Woo) {
        let f: child.aleo/Foo = child.aleo/Foo {bar: [child.aleo/Bar {baz: [child.aleo/Baz {one: child.aleo/One {two: [child.aleo/Two {val1: 1u32, val2: 2u32}, child.aleo/Two {val1: 3u32, val2: 4u32}]}}, child.aleo/Baz {one: child.aleo/One {two: [child.aleo/Two {val1: 5u32, val2: 6u32}, child.aleo/Two {val1: 7u32, val2: 8u32}]}}]}]};
        let (f1, b1): (child.aleo/Foo, child.aleo/Boo) = child.aleo/create();
        return (f1, b1, BooHoo {owner: self.signer, val: 10u32, woo: Woo {a: 1u32, b: 2u32}}, Woo {a: 3u32, b: 4u32});
    }

}

// --- Next Program --- //
import parent.aleo;
import child.aleo;
program grandparent.aleo {

    fn main(b: u32) -> u32 {
        let a: u32 = 1u32 + b;
        return a;
    }

    fn omega_wrapper() -> (child.aleo/Foo, child.aleo/Boo, parent.aleo/BooHoo, parent.aleo/Woo) {
        let (f, b, bh, w): (child.aleo/Foo, child.aleo/Boo, parent.aleo/BooHoo, parent.aleo/Woo) = parent.aleo/create_another_wrapper();
        return parent.aleo/create_another_wrapper();
    }
}
