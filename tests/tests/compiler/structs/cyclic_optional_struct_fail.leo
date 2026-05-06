// Regression: a struct that recurses through `Option<Self>` must be diagnosed
// as a cyclic composite dependency, not crash type-checking with a stack overflow.
// Before the fix, the cycle-graph edge insertion in `type_checking::program` had
// no `Type::Optional` arm, and `disallowed_inside_optional` had no visited-set,
// so declaring this struct hung the type checker until the main thread overflowed.
struct Node {
    val: u32,
    next: Node?,
}

program test.aleo {
    @noupgrade
    constructor() {}

    fn touch() -> u32 {
        return 0u32;
    }
}
