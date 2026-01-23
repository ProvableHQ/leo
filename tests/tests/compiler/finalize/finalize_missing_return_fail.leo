
program test.aleo {

    mapping account: address => u64;

    fn mint_public(public receiver: address, public amount: u64) -> Future {
        return fin_mint(receiver, amount);
    }
}

onchain fn fin_mint(public receiver: address, public amount: u64) -> u64 {
    Mapping::set(account, receiver, amount);
}
