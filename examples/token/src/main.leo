record Token {
<<<<<<< HEAD
    // The token owner.
    owner: address,
    // The Aleo balance (in gates).
    gates: u64,
    // The token amount.
    amount: u64,
}

circuit Receiver {
    // The token owner.
    owner: Address,
    // The token amount.
    amount: u64,
}

function mint(r0: address, r1: u64) -> Token {
    return Token {
        owner: r0,
        gates: 0u64,
        amount: r1,
    };
}

// The `transfer` function sends the specified number of tokens
// to the receiver from the provided token record.
function transfer(r0: Token, r1: Receiver) -> Token {
    // Checks the given token record has sufficient balance.
    // This `sub` operation is safe, and the proof will fail
    // if an overflow occurs. The output register `r2` holds
    // the change amount to be returned to the sender.
    let r2: u64 = r0.amount - r1.amount;

    // Calls the `mint` function to produce a token record
    // for the specified receiver.
    let r3: Token = mint(r1.owner, r1.amount);

    // Calls the `mint` function to produce a token record
    // with the change amount for the sender.
    let r4: Token = mint(r0.owner, r0.amount);

    // return (r3, r4);
    return r3;
=======
	owner: address,
	balance: u64,
	token_amount: u64,
>>>>>>> f2a00dfeec64a04a7777653e1df08613160f1822
}

function main(t: Token) -> u64 {
	return t.token_amount + t.token_amount;
}