// `@no_inline` on a `view fn` is meaningless: views are externally callable entry points
// (and only inline `Fn` callees inside their bodies). Pin the current behavior so a future
// change is forced to update this expectation.
program test.aleo {
    @no_inline
    view fn lookup(x: u64) -> u64 {
        return x + 1u64;
    }

    fn touch(x: u32) -> u32 {
        return x;
    }

    @noupgrade
    constructor() {}
}
