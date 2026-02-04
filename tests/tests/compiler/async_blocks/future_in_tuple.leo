program credits.aleo {
    record credits {
        owner: address,
        amount: u64,
    }

    fn transfer_private_to_public(input: credits, addr: address, amount:u64) -> (credits, Final) {
        let f: Final = final { assert_eq(1u8, 1u8); };
        return (input, f);
    }
}

// --- Next Program --- //

import credits.aleo;

program test_credits.aleo {
    fn send_credits(input: credits.aleo/credits, amount: u64) -> (credits.aleo/credits, Final) {
        let result: (credits.aleo/credits, Final) = credits.aleo/transfer_private_to_public(input, self.address, amount);
        return (result.0, final { result.1.run(); });
    }
}
