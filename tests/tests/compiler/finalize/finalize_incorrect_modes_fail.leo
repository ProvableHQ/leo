
program test.aleo {
    mapping account: address => u64;

    async fn mint_public(public receiver: address, public amount: u64) -> Future {
        return finalize_mint_public(receiver, amount);
    }

    async fn mint_public2(public receiver: address, public amount: u64) -> Future {
        return finalize_mint_public2(receiver, amount);
    }
}

async fn finalize_mint_public(public receiver: address, constant amount: u64) -> constant u64 {
    Mapping::set(account, receiver, amount);
}

async fn finalize_mint_public2(public receiver: address, amount: u64) -> u64 {
    Mapping::set(account, receiver, amount);
    return amount + amount;
}
