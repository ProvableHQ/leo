// Test: a library-based interface that requires a record type in a function parameter
// compiles correctly when the implementing function uses a different parameter name.
// Regression test for https://github.com/ProvableHQ/leo/issues/29327.

// --- library: test_lib --- //

interface Foo {
    record Token {
        owner: address,
        amount: u128,
        ..
    }

    fn transfer_private(token: Token);
}

// --- Next Program --- //

program test.aleo: test_lib::Foo {
    record Token {
        owner: address,
        amount: u128,
    }

    // `input` differs from the prototype's `token` — must compile without error.
    fn transfer_private(input: Token) {}

    @noupgrade
    constructor() {}
}
