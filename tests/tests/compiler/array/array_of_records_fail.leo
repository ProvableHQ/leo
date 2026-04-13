program test.aleo {
    record token {
        owner: address,
        amount: u64,
    }

    struct wrapper {
        data: [token; 2],
    }

    fn main(owner: address) -> u8 {
        let t: token = token { owner: owner, amount: 1u64 };
        let arr: [token; 2] = [t, t];
        return 0u8;
    }
}
