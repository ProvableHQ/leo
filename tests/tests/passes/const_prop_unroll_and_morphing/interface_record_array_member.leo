// Test: interface record prototype with an array-typed member field compiles without
// panicking. Regression test for the visit_record_prototype fix: the default
// visit_record_prototype was a no-op, leaving array-length literals inside record
// prototype member types without type table entries. TypeChecking would then panic in
// const propagation when it tried to evaluate those array-length expressions.

interface Vault {
    record Entry {
        owner: address,
        data: [u8; 32],
        ..
    }
    fn deposit(e: Entry) -> u64;
}

program test.aleo: Vault {
    record Entry {
        owner: address,
        data: [u8; 32],
    }

    fn deposit(e: Entry) -> u64 {
        return 0u64;
    }

    @noupgrade
    constructor() {}
}
