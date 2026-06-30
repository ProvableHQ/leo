@no_inline
export fn helper(a: u64, b: u64) -> u64 {
    return a + b;
}

program test.aleo {
    mapping account: address => u64;

    fn mint(public receiver: address, public amount: u64) -> Final {
        return final { finalize_mint(receiver, amount); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_mint(receiver: address, amount: u64) {
    let sum: u64 = helper(amount, 1u64);
    Mapping::set(account, receiver, sum);
}
