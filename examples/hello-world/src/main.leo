record Token {
	owner: address,
	balance: u64,
	token_amount: u64,
}

function main(r0: Token) -> u64 {
	return r0.token_amount + r0.token_amount;
}