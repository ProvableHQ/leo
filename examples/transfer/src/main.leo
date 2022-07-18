record Token {
	owner: address,
	gates: u64,
	amount: u64,
}

function main(owner: address, amount: u64) -> Token {
	return Token { 
		owner: owner,
		gates: 0u64,
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
		gates: 0u64,
		amount: a,
	};

	let t2: Token = Token {
		owner: t.owner,
		gates: t.gates,
		amount: change
	};

	return (t1, t2);
}