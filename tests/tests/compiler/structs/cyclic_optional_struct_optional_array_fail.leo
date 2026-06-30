// Regression for #29405: recursion via `[Self; N]?` (Optional of Array of Composite).
// Exercises Type::Optional(Type::Array(Type::Composite(_))) — the composite-graph edge
// insertion must traverse Optional → Array → Composite to record the self-edge.
export struct Node {
    kids: [Node; 1]?,
}

program test.aleo {
    @noupgrade
    constructor() {}
}
