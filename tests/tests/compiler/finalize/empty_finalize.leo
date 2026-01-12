
program test.aleo {

    fn mint_public(public receiver: address, public amount: u64) -> Final {
        return final { finalize_mint(receiver, amount); };
    }
}

final fn finalize_mint(public receiver: address, public amount: u64) {}
