
program test.aleo {

    mapping account: address => u64;

    fn mint_public(public receiver: address, public amount: u64) -> Future {
        return finalize_mint_public(receiver, amount, amount);
    }
}

onchain fn finalize_mint_public (public receiver: address, public amount: u64) {
    Mapping::set(account, receiver, amount);
}
