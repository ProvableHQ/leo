// Test: implementing program provides a record named Coin but the interface (in a
// library module) requires a record named Token. Two errors should fire:
//   1. missing_interface_record for Token (program has no record named Token).
//   2. signature_mismatch for mint: Coin is not Token, and type_eq_with_record_names
//      rejects the mismatch because Coin's name is not in record_names {Token}.

// --- library: token_lib --- //

// --- Next Module: itoken.leo --- //

interface TokenInterface {
    record Token;
    fn mint(owner: address) -> Token;
}

// --- Next Program --- //

program test.aleo: token_lib::itoken::TokenInterface {
    record Coin {
        owner: address,
        balance: u64,
    }

    fn mint(owner: address) -> Coin {
        return Coin { owner: owner, balance: 0u64 };
    }
}
