
program test.aleo {
    mapping account: address => u64;
    mapping values: u8 => u8;

    async fn mint_public(public receiver: address, public amount: u64) -> Future {
        return finalize_mint_public(receiver, amount);
    }
    async fn public_adder(public a: u8, public b: u8) -> Future {
        return finalize_public_adder(a, b);
    }
    async fn finalize_no_params() -> Future {
        return finalize_no_params();
    }
}

async fn finalize_mint_public(public receiver: address, public amount: u64) {
    increment(account, receiver, amount);
}

async fn finalize_public_adder(a: u8, b: u8) -> public u8 {
    return a + b;
}

async fn finalize_no_params() {
    increment(values, 0u8, 1u8);
    increment(account, self.caller, 1u64);
}
