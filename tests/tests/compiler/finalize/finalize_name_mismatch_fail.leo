program test.aleo {
    mapping account: address => u64;
    mapping values: u8 => u8;

    fn mint_public(public receiver: address, public amount: u64) -> Final {
        return final { finalize_mint_public(receiver, amount); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_mint_private(receiver: address, amount: u64) {
    Mapping::set(account, receiver, amount);
}
