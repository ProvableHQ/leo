// FAIL: records lower to a `.record` marker and `Final`s to a `.future`, neither of which carries a
// visibility, so a `public` / `private` / `constant` mode on a record or `Final` input or output of
// an entry point is meaningless and is rejected. This covers every entry-point position (record
// inputs, record outputs, `dyn record` inputs/outputs, `Final` inputs/outputs across all three
// modes), and then confirms the rule is scoped to entry points: on a `view fn`, a helper `fn`, and a
// `final fn`, the existing variant-specific diagnostics fire instead of this one.
//
// (A `constant` input additionally trips the entry-point const-input rule, and a `Final` input
// additionally trips the "`Final`s cannot be parameters here" rule; both extra errors are expected.)
program test.aleo {
    mapping Yo: u32 => u32;

    record Token {
        owner: address,
        amount: u64,
    }

    // Record inputs with a mode (entry point).
    fn rec_in_public(public t: Token) -> u64 { return t.amount; }
    fn rec_in_private(private t: Token) -> u64 { return t.amount; }
    fn rec_in_constant(constant t: Token) -> u64 { return t.amount; }

    // Record outputs with a mode (entry point).
    fn rec_out_public(owner: address, amount: u64) -> public Token { return Token { owner, amount }; }
    fn rec_out_private(owner: address, amount: u64) -> private Token { return Token { owner, amount }; }
    fn rec_out_constant(owner: address, amount: u64) -> constant Token { return Token { owner, amount }; }

    // `dyn record` inputs/outputs with a mode (the dynamic-record form of the same rule).
    fn dyn_in(public t: dyn record) -> u8 { return 1u8; }
    fn dyn_out(t: dyn record) -> public dyn record { return t; }

    // `Final` input with a mode (entry point).
    fn fut_in(public f: Final) -> u8 { return 1u8; }

    // `Final` outputs with a mode (entry point).
    fn fut_out_public() -> public Final { return final { fin(1u32, 1u32); }; }
    fn fut_out_private() -> private Final { return final { fin(1u32, 1u32); }; }
    fn fut_out_constant() -> constant Final { return final { fin(1u32, 1u32); }; }

    // Non-entry position: view fn input/output modes are rejected as view-mode errors, not the
    // record/`Final` mode error.
    view fn v(public x: u32) -> private u32 { return x; }

    @noupgrade
    constructor() {}
}

// Non-entry position: a helper `fn` with a moded record input gets the regular-`fn` mode error and
// the "only entry point fns can have a record" error, not the record/`Final` mode error.
export fn helper(public t: Token) -> u8 { return 1u8; }

// The finalize function invoked by the entry points' `final` blocks above.
export final fn fin(a: u32, b: u32) {
    Mapping::set(Yo, a, b);
}

// Non-entry position: a `final fn` with a moded `Final` input gets the final-fn input-mode error
// (`Final`s are valid parameters there), not the record/`Final` mode error.
export final fn fin_moded(public f: Final) {}
