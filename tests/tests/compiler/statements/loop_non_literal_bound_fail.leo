program test.aleo {
    record Token {
        owner: address,
        amount: u64,
    }

    fn main(owner: address, amount: u64) -> Token {
        let max: u64 = 0u64;
        for i:u64 in 0u64..amount {
            max = i;
        }
        return Token {
            owner: owner,
            amount: amount,
        };
    }
}
