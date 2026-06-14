// Comprehensive coverage of unused-items warnings when building a *library*
// directly. Every top-level item — in the entry file and in submodules alike —
// is the library's public surface (a consumer reaches it as `my_lib::item` or
// `my_lib::submodule::item`), so item-level fns, consts, and structs must NOT
// warn (their callers live in other compilation units). The body-level checks
// still apply: unused local bindings, loop variables, and the per-binding-site
// scope rules all fire regardless of where the enclosing function lives.

// Top-level fn with no local caller. Public surface — must NOT warn even though
// `call_count == 0` here.
fn exported_no_local_caller(x: u32) -> u32 {
    return x + 1u32;
}

// Top-level fn with an unused `let` inside. The function name is silent (public
// surface), but `dead` must warn.
fn fn_with_dead_local(x: u32) -> u32 {
    let dead: u32 = 99u32;
    return x;
}

// Top-level fn with an unused iter var. Must warn on `i`.
fn fn_with_dead_iter(x: u32) -> u32 {
    for i: u32 in 0u32..3u32 {
        let _t: u32 = x;
    }
    return x;
}

// Two sequential same-name loops where only the second reads `k`. The first
// `k` must still warn — regression for the per-binding-site scope fix.
fn shadow_scope(x: u32) -> u32 {
    for k: u32 in 0u32..3u32 {
        let _t: u32 = x;
    }
    let acc: u32 = x;
    for k: u32 in 0u32..3u32 {
        acc = acc + k;
    }
    return acc;
}

// `_`-prefixed top-level fn. Always-inlined; must not warn.
fn _intentionally_silent() -> u32 {
    return 1u32;
}

// Top-level fn that uses a submodule helper. Both should be silent.
fn calls_submodule(x: u32) -> u32 {
    return helpers::used_helper(x);
}

// Top-level fn that uses a submodule constant. Const should be silent.
fn reads_sub_const() -> u32 {
    return helpers::SUB_USED;
}

// Top-level fn that uses an `_`-prefixed name from a path (rejected? accepted? —
// just covers the parser change). `_h` is force-inlined and silent on its own.
fn calls_underscore_helper(x: u32) -> u32 {
    return _intentionally_silent() + x;
}

// Top-level const, never referenced inside the lib. Public surface — must NOT warn,
// matching how we treat top-level fns: consumers can read `my_lib::UNREFERENCED_CONST`.
const UNREFERENCED_CONST: u32 = 1u32;

// Top-level const, used by a top-level fn — must NOT warn.
const USED_CONST: u32 = 2u32;
fn reads_top_const() -> u32 {
    return USED_CONST;
}

// Top-level struct, never constructed inside the lib. Public surface — must NOT warn.
struct UnconstructedStruct {
    a: u32,
    b: u32,
}

// Top-level struct, used in a top-level fn signature — must NOT warn.
struct LiveStruct {
    a: u32,
}
fn uses_live_struct(s: LiveStruct) -> u32 {
    return s.a;
}
