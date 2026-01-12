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
        let start: (credits.aleo/credits, Final) = credits.aleo/transfer_private_to_public(input, self.address, amount);
        let after: (u8, credits.aleo/credits) = (1u8, start.0);
        let start_2: Final = start.1;
        return (after.1, final { finish(start.1, start_2); });
    }
}

final fn finish(f1: Final, f2: Final) {
    f1.run();
    f2.run();
}
