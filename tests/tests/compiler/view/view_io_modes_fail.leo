// FAIL: views run off-consensus; explicit `public` / `private` / `constant` modifiers on
// view fn inputs and outputs are not meaningful and are rejected up front. The default
// `Mode::None` is accepted and lowers to `.public` at the bytecode level.
program test.aleo {
    fn dummy(x: u32) -> u32 { return x; }

    view fn echo_private(private x: u32) -> u32 {
        return x;
    }

    view fn echo_public(public y: u32) -> u32 {
        return y;
    }

    view fn echo_constant(constant z: u32) -> u32 {
        return z;
    }

    @noupgrade
    constructor() {}
}
