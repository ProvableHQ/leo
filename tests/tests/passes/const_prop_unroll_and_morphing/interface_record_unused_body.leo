// Test: interface requires a record that is declared in the implementing program but
// never referenced inside any function body. The record must survive monomorphization
// (which prunes unused composites from the composite graph based on function body
// usage) and be visible to CheckInterfaces at fixed-point convergence.
//
// Regression test for the iter_records fix in TypeChecking: without it,
// monomorphization would drop Token from the composite graph (since no function body
// references it), and CheckInterfaces at convergence would erroneously report
// Token as missing.

interface Issuer {
    record Token {
        owner: address,
        ..
    }
    fn total_supply() -> u64;
}

program test.aleo: Issuer {
    record Token {
        owner: address,
        amount: u64,
    }

    // Token is declared but never referenced in this function's body.
    fn total_supply() -> u64 {
        return 1000000u64;
    }

    @noupgrade
    constructor() {}
}
