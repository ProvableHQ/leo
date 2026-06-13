// Submodule items are public surface too: a consumer reaches them as
// `my_lib::helpers::item`, so their call/read sites live in other compilation
// units. Like top-level items, unused submodule fns/consts must NOT warn in a
// library build. Unused *locals* inside those fns are still reported.

// Used by `lib.leo::calls_submodule`.
fn used_helper(x: u32) -> u32 {
    return x + 1u32;
}

// Submodule fn with no local caller. Public surface — must NOT warn.
fn dead_helper(x: u32) -> u32 {
    return x;
}

// Submodule fn with a dead local inside. The fn is public surface (no warning),
// but `dead_sub_local` must warn.
fn dead_helper_with_dead_local(x: u32) -> u32 {
    let dead_sub_local: u32 = 0u32;
    return x;
}

// `_`-prefixed submodule fn — force-inlined, never reaches the VM.
fn _silent_sub_helper() -> u32 {
    return 0u32;
}

// Submodule const, read by `lib.leo::reads_sub_const`.
const SUB_USED: u32 = 5u32;

// Submodule const with no local reader. Public surface — must NOT warn.
const SUB_UNUSED: u32 = 6u32;
