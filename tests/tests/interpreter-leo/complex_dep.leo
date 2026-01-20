struct Nested {
    a: alpha::SharedName,
    b: beta::SharedName::[beta::X],
    c: gamma::Inner,
}

program test.aleo {
    fn main() -> u32 {
        // Constant name shadowing (alpha::X vs beta::X vs gamma::X)
        let x1 = alpha::X;
        let x2 = beta::X;
        let x3 = gamma::X;

        // Struct name shadowing (SharedName)
        let s1 = alpha::SharedName {
            val: 1,
        };
        let s2 = beta::SharedName::[beta::X] {
            val: [2; beta::X],
        };

        // Cross-module inline chaining via main
        let g = gamma::make_inner(alpha::X + beta::X);
        let result = gamma::consume_inner(g);

        // Struct with field type from another module
        let composed = Nested {
            a: s1,
            b: s2,
            c: g,
        };

        // Inline fn call that takes struct from another module
        return x1 + x2 + x3
             + s2.val[1]
             + result;
    }
}

// --- Next Module: alpha.leo --- //

// Constant named X
const X: u32 = 2;

// Struct with same name as in beta (to test shadowing)
struct SharedName {
    val: u32,
}

// Inline returning its own struct
fn make() -> SharedName {
    return SharedName { val: 99 };
}

// --- Next Module: beta.leo --- //

// Constant shadowing X from alpha
const X: u32 = 3;

// Struct with same name but generic
struct SharedName::[N: u32] {
    val: [u32; N],
}

// Inline using generic struct
fn make() -> SharedName::[X] {
    return SharedName::[X] {
        val: [5; X],
    };
}

// --- Next Module: gamma.leo --- //

// Constant shadowing again
const X: u32 = 4;

// Struct referencing another module’s const and struct in main
struct Inner {
    n: u32,
}

// Inline returning struct defined here
fn make_inner(n: u32) -> Inner {
    return Inner { n };
}

fn consume_inner(inner: Inner) -> u32 {
    return inner.n + X;
}
