// Test: a record prototype inside a library submodule interface with a non-address
// owner field is rejected.

// --- library: token_lib --- //

// --- Next Module: tokens.leo --- //

interface Mintable {
    record Token {
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
