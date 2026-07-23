// Regression (issue #29589): an exported struct that no function in its own program references
// must still be emitted to that program's bytecode, so an external program can import and use it.
// `child.aleo` never references `Point`; when compiled independently (the FromAleo dependency path)
// the struct must survive pruning, otherwise `parent.aleo` fails with "struct not defined". The
// FromLeo and FromAleo results must agree.

export struct Point {
    x: u32,
    y: u32,
}

program child.aleo {
    // Note: nothing here references Point.
    fn noop(x: u32) -> u32 {
        return x;
    }
    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    fn use_it(x: u32, y: u32) -> u32 {
        let p: child.aleo::Point = child.aleo::Point { x: x, y: y };
        return p.x + p.y;
    }
    @noupgrade
    constructor() {}
}
