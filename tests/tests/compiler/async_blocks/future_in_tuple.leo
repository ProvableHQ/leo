program credits.aleo {
    record credits {
        owner: address,
        amount: u64,
    }

    async transition transfer_private_to_public(input: credits, addr: address, amount:u64) -> (credits, Future) {
        let f: Future = async { assert_eq(1u8, 1u8); };
        return (input, f);
    }
}

// --- Next Program --- //

import credits.aleo;

program test_credits.aleo {
    async transition send_credits(input: credits.aleo/credits, amount: u64) -> (credits.aleo/credits, Future) {
        let result: (credits.aleo/credits, Future) = credits.aleo/transfer_private_to_public(input, self.address, amount);
        return (result.0, async { result.1.await(); });
    }
}
