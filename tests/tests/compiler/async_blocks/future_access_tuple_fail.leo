program credits.aleo {
    record credits {
        owner: address,
        amount: u64,
    }

    async transition transfer_private_to_public(input: credits, addr: address, amount:u64) -> (credits, Future) {
        let f: Future = finalize();
        return (input, f);
    }

    async function finalize() {
        assert_eq(1u8, 1u8);
    }
}

// --- Next Program --- //

import credits.aleo;

program test_credits.aleo {
    async transition send_credits(input: credits.aleo/credits, amount: u64) -> (credits.aleo/credits, Future) {
        let start: (credits.aleo/credits, Future) = credits.aleo/transfer_private_to_public(input, self.address, amount);
        let after: (u8, credits.aleo/credits) = (1u8, start.0);
        let start_2: Future = start.1;
        return (
            after.1, 
            async {
                start.1.await();
                start_2.await();
            }
        );
    }
}
