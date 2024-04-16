/*
namespace: Compile
expectation: Pass
*/

program child.aleo {
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

    record Boo {
        owner: address,
        val: u32,
    }

    transition create() -> (Foo, Boo) {
        return (Foo {bar: [Bar {baz: [Baz {one: One {two: [Two {val1: 1u32, val2: 2u32}, Two {val1: 3u32, val2: 4u32}]}}, Baz {one: One {two: [Two {val1: 5u32, val2: 6u32}, Two {val1: 7u32, val2: 8u32}]}}]}]}, Boo {owner: self.caller, val: 10u32});
    }
}

// --- Next Program --- //

import child.aleo;
program parent.aleo {

    record BooHoo {
        owner: address,
        val: u32,
        woo: Woo,
    }

    struct Woo {
        a: u32,
        b: u32,
    }

    transition create_wrapper() -> (Foo, child.aleo/Boo) {
        let f: Foo = Foo {bar: [Bar {baz: [Baz {one: One {two: [Two {val1: 1u32, val2: 2u32}, Two {val1: 3u32, val2: 4u32}]}}, Baz {one: One {two: [Two {val1: 5u32, val2: 6u32}, Two {val1: 7u32, val2: 8u32}]}}]}]};
        let (f1, b1): (Foo, child.aleo/Boo) = child.aleo/create();
        return child.aleo/create();
    }

    transition create_another_wrapper() -> (Foo, child.aleo/Boo, BooHoo, Woo) {
        let f: Foo = Foo {bar: [Bar {baz: [Baz {one: One {two: [Two {val1: 1u32, val2: 2u32}, Two {val1: 3u32, val2: 4u32}]}}, Baz {one: One {two: [Two {val1: 5u32, val2: 6u32}, Two {val1: 7u32, val2: 8u32}]}}]}]};
        let (f1, b1): (Foo, child.aleo/Boo) = child.aleo/create();
        return (f1, b1, BooHoo {owner: self.caller, val: 10u32, woo: Woo {a: 1u32, b: 2u32}}, Woo {a: 3u32, b: 4u32});
    }

}

// --- Next Program --- //
import parent.aleo;
import child.aleo;
program grandparent.aleo {

    transition main(b: u32) -> u32 {
        let a: u32 = 1u32 + b;
        return a;
    }

    transition omega_wrapper() -> (Foo, child.aleo/Boo, parent.aleo/BooHoo, Woo) {
        let (f, b, bh, w): (Foo, child.aleo/Boo, parent.aleo/BooHoo, Woo) = parent.aleo/create_another_wrapper();
        return parent.aleo/create_another_wrapper();
    }
}