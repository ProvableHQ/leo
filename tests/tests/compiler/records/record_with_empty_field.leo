program test.aleo {
    record Token {
        owner: address,
        amount: u64,
		foobar: [u32; 0],
    }

    transition main(o: address, a: u64) -> Token {
        return Token { owner: o, amount: a, foobar: [] };
    }
}
