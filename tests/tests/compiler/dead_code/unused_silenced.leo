// Mirror of `unused_warnings.leo`, but every dead item now uses the leading-`_`
// convention so that no `unused_*` warning fires. The test also exercises the
// non-`_` silencing paths the pass relies on:
//
//   * Records — public surface, never warned.
//   * Entry-point fns and their parameters — public surface, never warned.
//   * Composite-init shorthand `Foo { x }` — counts as a use of any same-named
//     global `const`, so `x` is NOT warned even without a `_` prefix.
//   * `final fn` — always inlined; `_`-prefix still permitted to suppress the
//     orphan warning.
//
// `_`-prefixed STRUCT and ENTRY-POINT names are intentionally absent: the
// type checker rejects those (ETYC0372193 / ETYC0372194) because their names
// would reach the Aleo bytecode. See `underscore_{struct_name,entry_point}_fail.leo`.

// External program imported (and actually used) by `test.aleo` below — no
// `unused_import` warning.
program ext.aleo {
    fn from_ext() -> u32 {
        return 7u32;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import ext.aleo;

// Live struct constructed in main via composite-init shorthand.
struct Holder {
    shorthand_val: u32,
}

// `_`-prefixed top-level const — silenced.
const _DEAD_TOP_CONST: u32 = 1u32;

// Const referenced only via composite-init shorthand below — counts as used
// even without a `_` prefix.
const shorthand_val: u32 = 99u32;

program test.aleo {
    // Records — public surface, never warned (no `_` prefix needed).
    record Token {
        owner: address,
        amount: u64,
    }

    mapping balances: address => u128;

    fn main(
        // Entry-point parameters are public surface — never warned, regardless
        // of `_` prefix.
        public used: u32,
        public unused_entry_param: u32,
    ) -> u32 {
        // `_let` binding — silenced.
        let _dead_let: u32 = 0u32;

        // `_const` local — silenced.
        const _DEAD_LOCAL: u32 = 0u32;

        // `_iter` loop variable — silenced.
        for _dead_iter: u32 in 0u32..1u32 {}

        // `_tup_elem` tuple-pattern element — silenced.
        let (used_tup, _dead_tup): (u32, u32) = pair(used);

        // Composite-init shorthand: `Holder { shorthand_val }` desugars to
        // `shorthand_val: shorthand_val`. The RHS resolves to the global
        // `const shorthand_val` above, which is therefore treated as used and
        // produces no warning.
        let h: Holder = Holder { shorthand_val };

        return used_tup
            + h.shorthand_val
            + helper::[3u32](used)
            + utils::module_helper(used)
            + ext.aleo::from_ext();
    }

    @noupgrade
    constructor() {}
}

// Orphan free fn — silenced by `_` prefix, force-inlined.
fn _dead_helper(x: u32) -> u32 {
    return x;
}

// Live free fn with `_`-prefixed parameters AND const-generic parameter — both silenced.
fn helper::[_DEAD_N: u32](_dead_param: u32) -> u32 {
    return 1u32;
}

// Orphan `final fn` with `_` prefix — silenced. Body deliberately avoids
// reading `_addr`/`_n` so the silencing marker isn't defeated.
final fn _orphan_final(_addr: address, _n: u128) {
    let _ignored: u32 = 0u32;
}

// Helper called from main — live.
fn pair(x: u32) -> (u32, u32) {
    return (x, x);
}

// --- Next Module: utils.leo --- //

// `_`-prefixed module-level const — silenced.
const _DEAD_MODULE_CONST: u32 = 1u32;

// Orphan module fn — silenced.
fn _dead_module_fn(x: u32) -> u32 {
    return x;
}

// Live module fn called from main, with `_`-prefixed body bindings — all silenced.
fn module_helper(x: u32) -> u32 {
    let _dead_module_local: u32 = 0u32;
    return x + 1u32;
}
