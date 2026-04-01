// The 'some_sample_leo_program' program.
program some_sample_leo_program.aleo {
    record Token {
        owner: address,
        amount: u64,
    }

    fn main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }

    fn mint(owner: address, amount: u64) -> Token {
        return Token { owner, amount };
    }

    fn dynamic_transfer(target: field, token: dyn record, to: address) -> dyn record {
        return _dynamic_call::[dyn record, address, dyn record](target, 'aleo', 'transfer', token, to);
    }

    @noupgrade
    constructor() {}
}
