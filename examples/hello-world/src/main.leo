
record Token {
    // The token owner.
    owner: address,
    // The Aleo balance (in gates).
    balance: u64,
    // The token amount.
    amount: u64,
}

function main(a: u8) -> group {
    return Pedersen64::hash(a);
}