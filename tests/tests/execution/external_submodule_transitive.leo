/*
[case]
program = "parent.aleo"
function = "sum_pixel"
input = ["{ r: 1u32, g: 2u32, b: 3u32 }"]

[case]
program = "parent.aleo"
function = "blend_and_sum"
input = []

[case]
program = "parent.aleo"
function = "pipeline"
input = ["10u32", "20u32", "30u32"]
*/

// Test: transitive 3-program chain.
//   grandchild.aleo — defines types::Pixel in a submodule
//   child.aleo      — imports grandchild, exposes a blend entry fn
//   parent.aleo     — imports both, uses grandchild's submodule types directly
//                     and calls child's entry that manipulates those types.

program grandchild.aleo {
    fn make_pixel(r: u32, g: u32, b: u32) -> types::Pixel {
        return types::Pixel { r: r, g: g, b: b };
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: types.leo --- //

struct Pixel {
    r: u32,
    g: u32,
    b: u32,
}

// --- Next Program --- //

import grandchild.aleo;

program child.aleo {
    // Takes two grandchild submodule types, adds them component-wise.
    fn blend(
        a: grandchild.aleo::types::Pixel,
        b: grandchild.aleo::types::Pixel,
    ) -> grandchild.aleo::types::Pixel {
        return grandchild.aleo::types::Pixel {
            r: a.r + b.r,
            g: a.g + b.g,
            b: a.b + b.b,
        };
    }

    // Scales a pixel's channels by a constant.
    fn scale_pixel(p: grandchild.aleo::types::Pixel, factor: u32) -> grandchild.aleo::types::Pixel {
        return grandchild.aleo::types::Pixel {
            r: p.r * factor,
            g: p.g * factor,
            b: p.b * factor,
        };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import grandchild.aleo;
import child.aleo;

program parent.aleo {
    // Direct access to grandchild's submodule type — sum channels.
    fn sum_pixel(p: grandchild.aleo::types::Pixel) -> u32 {
        return p.r + p.g + p.b;
    }

    // Build two pixels locally, blend via child, sum result.
    // Expected: (1+4) + (2+5) + (3+6) = 5+7+9 = 21
    fn blend_and_sum() -> u32 {
        let a: grandchild.aleo::types::Pixel = grandchild.aleo::types::Pixel { r: 1u32, g: 2u32, b: 3u32 };
        let b: grandchild.aleo::types::Pixel = grandchild.aleo::types::Pixel { r: 4u32, g: 5u32, b: 6u32 };
        let c: grandchild.aleo::types::Pixel = child.aleo::blend(a, b);
        return c.r + c.g + c.b;
    }

    // Full pipeline: make pixel via grandchild entry, scale via child, sum here.
    // Expected: (10+20+30) * 2 = 120
    fn pipeline(r: u32, g: u32, b: u32) -> u32 {
        let p: grandchild.aleo::types::Pixel = grandchild.aleo::make_pixel(r, g, b);
        let scaled: grandchild.aleo::types::Pixel = child.aleo::scale_pixel(p, 2u32);
        return scaled.r + scaled.g + scaled.b;
    }

    @noupgrade
    constructor() {}
}
