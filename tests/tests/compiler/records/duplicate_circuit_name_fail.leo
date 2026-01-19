
program test.aleo {
    record Token {
        // The token owner.
        owner: address,
        // The token amount.
        amount: u64,
    }

}

struct Token { // This struct cannot have the same name as the record defined above it.
    x: u32,
}

fn main(zz: bool) -> bool {
    return true;
}
