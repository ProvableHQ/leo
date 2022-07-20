record Token {
    // The token owner.
    owner: address,
    // The Aleo balance (in gates).
    gates: u64,
    // The token amount.
    amount: u64,
}

function main(t: Token) -> u64 {
	return t.amount + t.amount;
}

function mint(owner: address, amount: u64) -> Token {
    return Token {
        owner: owner,
        gates: 0u64,
        amount: amount,
    };
}

function transfer(token: Token, to: address, amount: u64) -> (Token, Token) {
    let difference: u64 = token.amount - amount;
    let remaining: Token = Token {
        owner: token.owner,
        gates: token.gates,
        amount: difference,
    };
    let transferred: Token = Token {
        owner: to,
        gates: 0u64,
        amount: amount,
    };
    return (remaining, transferred);
}