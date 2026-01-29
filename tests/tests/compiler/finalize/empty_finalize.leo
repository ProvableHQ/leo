
program test.aleo {

    fn mint_public(public receiver: address, public amount: u64) -> Final {
        return finalize_mint(receiver, amount);
    }
}

onchain fn finalize_mint(public receiver: address, public amount: u64) {}
