program test.aleo {
    record Token {
        owner: address,
        amount: u64,
    }

    fn mint(owner: address, amount: u64) {
        let t: Token = Token { owner, amount };
        let f = final {
            assert(t.owner == aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc);
        };
    }
}
