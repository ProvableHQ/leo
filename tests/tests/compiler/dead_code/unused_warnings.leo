// Comprehensive coverage of every UnusedItems warning. Items appear at every
// nesting level — external program, top-level (program scope), submodule, and
// inside function bodies — so the test also serves as a regression for the
// per-Location keying. Each labelled section emits at least one diagnostic;
// items marked "live" exist only to set up the surrounding fixture.

// External program providing one fn for the main program to call. Used to set
// up the `import ext.aleo;` chain below.
program ext.aleo {
    fn from_ext() -> u32 {
        return 7u32;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

// ---------- WUNU03714003 — unused import ----------
// `ext.aleo` is imported but never referenced from `test.aleo` below.
import ext.aleo;

// ---------- WUNU03714002 — unused struct + transitive deadness ----------

// Outer dead struct (nothing in user code references it).
struct DeadTopStruct {
    x: u32,
}

// Transitive deadness: A is referenced only by its own member type B; nothing
// in user code references either of them, so both must warn.
struct A {
    f: B,
}
struct B {
    x: u32,
}

// Live struct, constructed in main — no warning.
struct UsedStruct {
    a: u32,
}

// ---------- WUNU03714004 — unused top-level const ----------
// Consts live at the file top level (global scope); they are not permitted
// inside the `program { ... }` block.
const DEAD_TOP: u32 = 1u32;
const ALIVE_TOP: u32 = 2u32; // live

program test.aleo {
    // Records are program surface — never warn.
    record Token {
        owner: address,
        amount: u64,
    }

    mapping balances: address => u128;

    fn main(x: u32) -> u32 {
        // ---------- WUNU03714001 — unused `let` ----------
        let dead_let: u32 = 0u32;

        // ---------- WUNU03714004 — unused local `const` ----------
        const DEAD_LOCAL: u32 = 0u32;

        // ---------- WUNU03714001 — unused loop iterator ----------
        for dead_iter: u32 in 0u32..1u32 {}

        // ---------- WUNU03714001 — unused tuple-pattern element ----------
        let (used_tup, dead_tup): (u32, u32) = pair(x);

        return used_tup
            + ALIVE_TOP
            + UsedStruct { a: x }.a
            + helper::[3u32](x, x)
            + read_underscore(x)
            + utils::module_helper(x);
    }

    @noupgrade
    constructor() {}
}

// Live free fn (called from main). Demonstrates two more warning kinds:
//   - WUNU03714001 on `DEAD_N` (unused const-generic parameter)
//   - WUNU03714001 on `dead_param` (unused input parameter)
fn helper::[DEAD_N: u32](used: u32, dead_param: u32) -> u32 {
    return used;
}

// ---------- WUNU03714000 — unused free function ----------
fn orphan(x: u32) -> u32 {
    return x;
}

// ---------- WUNU03714000 — unused `final fn` ----------
final fn orphan_final(addr: address, n: u128) {
    balances.set(addr, n);
}

// ---------- WUNU03714005 — used underscore binding ----------
// Reading `_y` defeats the silencing marker.
fn read_underscore(_y: u32) -> u32 {
    return _y;
}

// Helper for the tuple-pattern test — live (called from main).
fn pair(x: u32) -> (u32, u32) {
    return (x, x);
}

// --- Next Module: utils.leo --- //

// ---------- Module-level coverage ----------
// The same warning kinds fire in submodules. Items called from the main program
// stay live; others trigger the same diagnostics as their top-level counterparts.

// WUNU03714004 — unused module-level const.
const DEAD_MODULE_CONST: u32 = 1u32;

// WUNU03714002 — unused module-level struct.
struct DeadModuleStruct {
    x: u32,
}

// Live module fn (called from main). Has its own unused local to also exercise
// body-level warnings in a module context.
fn module_helper(x: u32) -> u32 {
    // WUNU03714001 — unused local in a module function body.
    let dead_module_local: u32 = 0u32;
    return x + 1u32;
}

// WUNU03714000 — unused module-level function.
fn dead_module_fn(x: u32) -> u32 {
    return x;
}
