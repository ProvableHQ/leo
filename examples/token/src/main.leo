record Token {
	owner: address,
	balance: u64,
	token_amount: u64,
}

function main(t: Token) -> u64 {
	return t.token_amount + t.token_amount;
}