// Regression for #29405: recursion via `[Self?; N]` (Array of Optional of Composite).
// Exercises Type::Array(Type::Optional(Type::Composite(_))) — the composite-graph edge
// insertion must traverse Array → Optional → Composite to record the self-edge.
struct Node {
    kids: [Node?; 2],
}

program test.aleo {}
