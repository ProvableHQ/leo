
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

program multi_token_support_program.aleo {
    record Token {
        owner: address,
        amount: u64,
    }

    async transition mint_private() -> (Token, Future) {
        let f: Future = finalize();
        return (Token{owner: self.signer, amount: 100u64}, f);
    }

    async function finalize() {
        assert_eq(1u8, 1u8);
    }
}

// --- Next Program --- //

import credits.aleo;
import multi_token_support_program.aleo;

program mtsp_credits.aleo {
    async transition deposit_credits_private(
      input_record: credits.aleo/credits,
      amount: u64
    ) -> (credits.aleo/credits, multi_token_support_program.aleo/Token, Future) {
      let transfer_output: (credits.aleo/credits, Future) = credits.aleo/transfer_private_to_public(input_record, self.address, amount);
      let mint_output: (multi_token_support_program.aleo/Token, Future) = multi_token_support_program.aleo/mint_private();
      return (transfer_output.0, mint_output.0, finalize_deposit(transfer_output.1, mint_output.1));
    }

    // In an earlier Leo version, both Futures would be incorrectly inferred to
    // be the same type.
    async function finalize_deposit(f0: Future, f1: Future) {
      f0.await();
      f1.await();
    }
}
