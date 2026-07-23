program test.aleo {
    record test_credits {
        owner: address,
        amount: u64,
    }

    fn mint_credits(to: address, amount: u64) {
        return test_credits { owner: std::ctx::signer(), amount };
    }

    @noupgrade
    constructor() {}
}
