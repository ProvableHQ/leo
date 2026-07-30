// Records may have `impl` blocks, but a method that takes or returns the record type hits Leo's
// existing rule that only entry-point fns may have a record as input or output (impl methods are
// inlined helpers, just like any other helper fn). Static methods whose signatures avoid the
// record type work fine.
program test.aleo {
    record Token {
        owner: address,
        amount: u64,
    }

    impl Token {
        fn max_supply() -> u64 {
            return 1_000_000u64;
        }
    }

    fn main(a: u64) -> u64 {
        return a + Token::max_supply();
    }

    @noupgrade
    constructor() {}
}
