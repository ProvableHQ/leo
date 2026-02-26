
program test.aleo {

    mapping account: address => u64;

    fn mint_public(public receiver: address, public amount: u64) -> Final {
        return final { fin_mint(receiver, amount); };
    }
}

final fn fin_mint(public receiver: address, public amount: u64) -> u64 {
    Mapping::set(account, receiver, amount);
}
