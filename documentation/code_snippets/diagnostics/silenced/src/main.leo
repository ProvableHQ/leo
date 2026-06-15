// Every dead item in this file uses a leading `_` to silence its
// `unused_*` warning, so the project builds cleanly with no diagnostics.

// ANCHOR: top_const
const _UNUSED_TOP: u32 = 1u32;
// ANCHOR_END: top_const

// ANCHOR: free_fn
fn _orphan(x: u32) -> u32 {
    return x;
}
// ANCHOR_END: free_fn

program diagnostics_silenced.aleo {
    mapping balances: address => u128;

    fn main(public x: u32) -> u32 {
        // ANCHOR: let_binding
        let _scratch: u32 = 0u32;
        // ANCHOR_END: let_binding

        // ANCHOR: local_const
        const _LOCAL_MAX: u32 = 0u32;
        // ANCHOR_END: local_const

        // ANCHOR: loop_iter
        for _i: u32 in 0u32..1u32 {}
        // ANCHOR_END: loop_iter

        // ANCHOR: tuple_pattern
        let (used, _dropped): (u32, u32) = pair(x);
        // ANCHOR_END: tuple_pattern

        return used + helper::[3u32](x);
    }

    @noupgrade
    constructor() {}
}

// ANCHOR: const_generic_and_param
fn helper::[_N: u32](_unused: u32) -> u32 {
    return 1u32;
}
// ANCHOR_END: const_generic_and_param

fn pair(x: u32) -> (u32, u32) {
    return (x, x);
}
