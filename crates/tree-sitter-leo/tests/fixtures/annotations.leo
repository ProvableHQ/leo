@program(address = "aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqp7x0l")
fn main(public caller: address) -> u32 {
    return 1u32;
}

@test(mapping = "account")
final fn finalize(receiver: address, amount: u64) -> Final {
    return final {
        assert(amount > 0u64);
    };
}

program token.aleo {
    @noupgrade
    constructor() {}
}
