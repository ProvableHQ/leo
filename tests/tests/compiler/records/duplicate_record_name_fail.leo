program test.aleo {
    record Token {
        owner: address,
        amount: u64,
    }

    record Token {
        owner: address,
        balance: u64,
    }

    transition main() -> u32 {
        return 0u32;
    }
}

