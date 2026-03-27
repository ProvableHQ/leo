// Test: a library interface record prototype that declares `owner` with a type
// other than `address` should be rejected.

// --- library: bad_owner_lib --- //

interface HasToken {
    record Token {
        owner: address,
        ..
    }

    record BadToken {
        owner: u32,  // must be `address`
        ..
    }
}

// --- Next Program --- //

program main.aleo {
    fn run() -> u32 {
        return 0u32;
    }

    @noupgrade
    constructor() {}
}
