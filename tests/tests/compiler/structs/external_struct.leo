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

    transition create_wrapper() -> (Foo, child.aleo/Boo) {
        let f: Foo = Foo {bar: [Bar {baz: [Baz {one: One {two: [Two {val1: 1u32, val2: 2u32}, Two {val1: 3u32, val2: 4u32}]}}, Baz {one: One {two: [Two {val1: 5u32, val2: 6u32}, Two {val1: 7u32, val2: 8u32}]}}]}]};
        let (f1, b1): (Foo, child.aleo/Boo) = child.aleo/create();
        return child.aleo/create();
    }
}