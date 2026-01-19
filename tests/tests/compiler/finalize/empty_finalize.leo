
program test.aleo {

    async fn mint_public(public receiver: address, public amount: u64) -> Future {
        return finalize_mint(receiver, amount);
    }
}

async fn finalize_mint(public receiver: address, public amount: u64) {}
