// Test: accessing a struct from a two-level deep submodule of an imported .aleo program.
// Exercises 3-segment locator syntax: child.aleo::outer::inner::Type.

program child.aleo {
    fn make_point_deep(x: u32, y: u32) -> outer::inner::DeepPoint {
        return outer::inner::DeepPoint { x: x, y: y };
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: outer/inner.leo --- //

struct DeepPoint {
    x: u32,
    y: u32,
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    // 3-segment type annotation.
    fn use_deep_type(p: child.aleo::outer::inner::DeepPoint) -> u32 {
        return p.x + p.y;
    }

    // 3-segment struct literal.
    fn make_deep_struct(x: u32, y: u32) -> child.aleo::outer::inner::DeepPoint {
        return child.aleo::outer::inner::DeepPoint { x: x, y: y };
    }

    // Call a child entry that returns a 3-segment struct.
    fn call_child_deep(x: u32, y: u32) -> child.aleo::outer::inner::DeepPoint {
        return child.aleo::make_point_deep(x, y);
    }

    @noupgrade
    constructor() {}
}
