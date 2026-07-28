// Regression (issue #29589): an exported struct in a submodule that no function in this program
// references must still be emitted to bytecode, because external programs may import and use it.
// Compiled alone, `Point` must appear in the output rather than being pruned as dead code.

program child.aleo {
    fn noop(x: u32) -> u32 {
        return x;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: types.leo --- //

export struct Point {
    x: u32,
    y: u32,
}
