
program test.aleo {
    mapping account: address => u64;

    fn mint_public(public receiver: address, public amount: u64) -> Final {
        return final { finalize_mint_public(receiver, amount); };
    }

    fn mint_public2(public receiver: address, public amount: u64) -> Final {
        return final { finalize_mint_public2(receiver, amount); };
    }
}

final fn finalize_mint_public(public receiver: address, constant amount: u64) -> constant u64 {
    Mapping::set(account, receiver, amount);
}

final fn finalize_mint_public2(public receiver: address, amount: u64) -> u64 {
    Mapping::set(account, receiver, amount);
    return amount + amount;
}
