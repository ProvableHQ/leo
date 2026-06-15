program test.aleo {
    fn mint_public(public receiver: address, public amount: u64) -> Final {
        return final { finalize_mint(receiver, amount); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_mint(_receiver: address, _amount: u64) {}
