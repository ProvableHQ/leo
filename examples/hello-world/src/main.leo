
circuit Token {
    // The token owner.
    balance: u64,
    data: u64
}

function mint() -> Token {
    let tok: Token = Token { balance: 1u64, data: 1u64};
    return tok;
}

function main(a: u8) -> group {

    return Pedersen64::hash(a);
}