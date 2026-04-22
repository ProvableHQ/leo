// Test: a library top-level struct used as a storage type in the main program.
//
// Regression test for issue #29353: the option-lowering pass panicked at
// `visitor.rs:133` ("must exist by construction") when a library top-level struct
// was used as the type of a storage variable and `.unwrap()` was called on it.
// The cause was that `OptionLoweringVisitor::reconstruct_library` did not
// pre-populate the pass's composite registry with the library's top-level
// structs, so `wrap_none`'s zero-value construction could not find the
// members. Phase 1 (`collect_composites_from_program`) now walks stubs
// recursively and registers every composite in `self.composites`.

// --- library: test_lib --- //

struct TokenInfo {
    name: u128,
    symbol: u128,
    decimals: u8,
    supply: u128,
    max_supply: u128,
}

interface Foo {
    storage token_info: TokenInfo;

    fn unwrap_info() -> Final;
}

// --- Next Program --- //

program test.aleo : test_lib::Foo {
    storage token_info: test_lib::TokenInfo;

    fn unwrap_info() -> Final {
        return final {
            let info = token_info.unwrap();
            assert_eq(info.decimals, 8u8);
        };
    }

    @noupgrade
    constructor() {}
}
