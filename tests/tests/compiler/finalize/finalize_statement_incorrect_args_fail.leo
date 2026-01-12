
program test.aleo {

    mapping account: address => u64;

    fn mint_public(public receiver: address, public amount: u64) -> Final {
        return final { finalize_mint_public(receiver, amount, amount); };
    }
}

final fn finalize_mint_public (public receiver: address, public amount: u64) {
    Mapping::set(account, receiver, amount);
}
