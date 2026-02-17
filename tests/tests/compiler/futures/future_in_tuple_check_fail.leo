program credits.aleo {
    record credits {
        owner: address,
        amount: u64,
    }

    fn transfer_private_to_public(input: credits, addr: address, amount:u64) -> (credits, Final) {
        let f: Final = final { finalize(); };
        return (input, f);
    }
}

final fn finalize() {
    assert_eq(1u8, 1u8);
}

// --- Next Program --- //

import credits.aleo;

program test_credits.aleo {
    fn send_credits(input: credits.aleo/credits, amount: u64) -> (credits.aleo/credits, Final) {
        let result: (credits.aleo/credits, Final) = credits.aleo/transfer_private_to_public(input, self.address, amount);
        let result2: (credits.aleo/credits, Final) = credits.aleo/transfer_private_to_public(input, self.address, amount);
        return (result.0, final { finish(result.1); });
    }
}

final fn finish(f: Final) {
    f.run();
}
