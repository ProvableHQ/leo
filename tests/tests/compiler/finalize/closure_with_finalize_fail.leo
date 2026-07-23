program test.aleo {
    mapping account: address => u64;

    @noupgrade
    constructor() {}
}

export fn foo(a: u8, b: u8) -> Final {
    return finalize_bar(a, b);
}

export fn bar(a: u8, b: u8) -> u8 {
    return a + b;
}

export final fn finalize_bar(a: u8, b: u8) -> u8 {
    return a + b;
}

export fn mint_public(receiver: address, amount: u64) -> Final {
    return final { finalize_mint(receiver, amount); };
}

export final fn finalize_mint(receiver: address, amount: u64) {
    Mapping::set(account, receiver, amount);
}
