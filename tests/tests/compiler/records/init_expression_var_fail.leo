
program test.aleo {
    record Token {
        // The token owner.
        owner: address,
        // The token amount.
        amount: u64,
    }
}

@no_inline
fn mint(r0: address, r1: u64) -> Token {
    return Token {
        sender: r0, // This variable should be named `owner`.
        amount: r1,
    };
}

fn main(x: address) -> u64 {
    let c: u64 = 1u64;
    let t: Token = mint(x, c);

    return c;
}


