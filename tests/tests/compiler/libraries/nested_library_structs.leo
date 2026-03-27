// Test: nested library dependencies with generic structs.
//
// Dependency graph:
//   math_lib          (library: generic struct Foo, concrete struct Bar, consts OFFSET/SCALE)
//   config_lib        (library: consts BASE/FACTOR, depends on math_lib)
//   helper.aleo       (program: uses math_lib structs and both library consts; has module dep.leo)
//   main.aleo         (program: imports helper.aleo, uses math_lib structs and library consts)
//
// Verifies:
//  - Generic library structs (Foo::[N]) are monomorphized at each distinct call site.
//  - Concrete library structs (Bar containing Foo::[42]) work transitively.
//  - Library consts are resolved at compile time and not emitted as bytecode imports.
//  - A program can use the same generic library struct at multiple different sizes.
//  - Modules inside a program compile alongside library struct usage.

// --- library: math_lib --- //

struct Foo::[N: u32] {
    x: [u32; N + 1],
}

struct Bar {
    f: Foo::[42],
}

const OFFSET: u32 = 10u32;
const SCALE: u32 = OFFSET + OFFSET;

// --- Next Program --- //

// --- library: config_lib --- //

const BASE: u32 = math_lib::OFFSET + 5u32;
const FACTOR: u32 = math_lib::SCALE + BASE;

// --- Next Program --- //

program helper.aleo {
    fn add_offset(x: u32) -> u32 {
        let f = math_lib::Foo::[99] { x: [12; 100] };
        let b = math_lib::Bar {
            f: math_lib::Foo::[42] { x: [4; 43] },
        };
        return x + config_lib::BASE + math_lib::OFFSET + f.x[42] + b.f.x[3];
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: dep.leo --- //
struct B::[N: u32] {
    z: u32,
}

// --- Next Program --- //
import helper.aleo;

struct Foo {
    y: u32,
}

program main.aleo {
    fn compute(x: u32) -> (u32, math_lib::Foo::[43], Foo) {
        let a: u32 = helper.aleo::add_offset(x);
        let b: math_lib::Bar = math_lib::Bar {
            f: math_lib::Foo::[42] { x: [1000; 43] },
        };
        return (
            b.f.x[0] + a + config_lib::FACTOR + math_lib::SCALE,
            math_lib::Foo::[43] { x: [99; 44] },
            Foo { y: 199 },
        );
    }

    @noupgrade
    constructor() {}
}
