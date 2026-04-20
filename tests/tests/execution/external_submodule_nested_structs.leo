/*
[case]
program = "parent.aleo"
function = "sum_outer"
input = ["{ inner: { val: 7u32 }, count: 3u32 }"]

[case]
program = "parent.aleo"
function = "make_and_sum"
input = ["3u32", "4u32", "5u32"]

[case]
program = "parent.aleo"
function = "nest_and_double"
input = ["10u32"]
*/

// Test: accessing structs from a submodule where one struct contains a field
// typed as another struct from the same submodule (struct-depends-on-struct).

program child.aleo {
    fn make_outer(x: u32, y: u32, n: u32) -> types::Outer {
        let inner: types::Inner = types::Inner { val: x + y };
        return types::Outer { inner: inner, count: n };
    }

    fn double_outer(o: types::Outer) -> types::Outer {
        let doubled_inner: types::Inner = types::Inner { val: o.inner.val * 2u32 };
        return types::Outer { inner: doubled_inner, count: o.count * 2u32 };
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: types.leo --- //

struct Inner {
    val: u32,
}

struct Outer {
    inner: Inner,
    count: u32,
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    // Take a submodule struct (whose field is another submodule struct) and sum its fields.
    fn sum_outer(o: child.aleo::types::Outer) -> u32 {
        return o.inner.val + o.count;
    }

    // Call a child entry fn that builds and returns a nested struct, then sum.
    fn make_and_sum(x: u32, y: u32, n: u32) -> u32 {
        let o: child.aleo::types::Outer = child.aleo::make_outer(x, y, n);
        return o.inner.val + o.count;
    }

    // Chain: build nested struct locally, pass to child for doubling, then sum.
    fn nest_and_double(v: u32) -> u32 {
        let o: child.aleo::types::Outer = child.aleo::types::Outer {
            inner: child.aleo::types::Inner { val: v },
            count: v,
        };
        let doubled: child.aleo::types::Outer = child.aleo::double_outer(o);
        return doubled.inner.val + doubled.count;
    }

    @noupgrade
    constructor() {}
}
