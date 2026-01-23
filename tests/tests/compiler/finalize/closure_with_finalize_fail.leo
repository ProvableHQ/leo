
program test.aleo {
    mapping account: address => u64;
}

fn foo(a: u8, b: u8) -> Future {
    return finalize_bar(a, b);
}

fn bar(a: u8, b: u8) -> u8 {
    return a + b;
}

onchain fn finalize_bar(a: u8, b: u8) -> u8 {
    return a + b;
}


fn mint_public(receiver: address, amount: u64) -> Future {
    return finalize_mint(receiver, amount);
}

onchain fn finalize_mint(receiver: address, amount: u64) {
    Mapping::set(account, receiver, amount);
}
