
program test.aleo {

    mapping account: address => u64;

    async fn mint_public(public receiver: address, public amount: u64) -> Future {
        return finalize_mint_public(receiver, amount, amount);
    }
}

async fn finalize_mint_public (public receiver: address, public amount: u64) {
    Mapping::set(account, receiver, amount);
}
