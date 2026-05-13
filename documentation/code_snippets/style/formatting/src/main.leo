// ANCHOR: braces
struct A {
    x: u32,
    y: u32,
}

fn main() -> A {
    let a: A = A { x: 0u32, y: 0u32 };
    return a;
}
// ANCHOR_END: braces

program style_formatting.aleo {
    fn semicolons_demo() -> u32 {
        // ANCHOR: semicolons
        let a: u32 = 1u32;
        let b: u32 = a + 5u32;
        b *= 2u32;

        return b;
        // ANCHOR_END: semicolons
    }

    fn commas_demo() {
        // ANCHOR: commas
        let single: A = A { x: 0u32, y: 1u32 };

        let multi: A = A {
            x: 0u32,
            y: 1u32,
        };
        // ANCHOR_END: commas
    }

    @noupgrade
    constructor() {}
}
