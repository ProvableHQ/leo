record Token {
	owner: address,
	balance: u64,
	amount: u64,
}

function main(owner: address, amount: u64) -> Token {
	return Token { 
		owner: owner,
		balance: 0u64,
		amount: amount,
	};
}

function transfer(
	t: Token, 
	r: address, 
	a: u64
) -> (Token, Token) {
	let change: u64 = t.amount - a;

	let t1: Token = Token {
		owner: r,
		balance: 0u64,
		amount: a,
	};

	let t2: Token = Token {
		owner: t.owner,
		balance: t.balance,
		amount: change
	};

	return (t1, t2);
}