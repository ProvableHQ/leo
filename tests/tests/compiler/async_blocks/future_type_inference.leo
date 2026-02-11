
program credits.aleo {
    record credits {
        owner: address,
        amount: u64,
    }

    fn transfer_private_to_public(input: credits, addr: address, amount:u64) -> (credits, Final) {
        return (input, final { assert_eq(1u8, 1u8); });
    }
}

// --- Next Program --- //

program multi_token_support_program.aleo {
    record Token {
        owner: address,
        amount: u64,
    }

    fn mint_private() -> (Token, Final) {
        let f: Final = final { assert_eq(1u8, 1u8); };
        return (Token{owner: self.signer, amount: 100u64}, f);
    }
}

// --- Next Program --- //

import credits.aleo;
import multi_token_support_program.aleo;

program mtsp_credits.aleo {
    fn deposit_credits_private(
      input_record: credits.aleo/credits,
      amount: u64
    ) -> (credits.aleo/credits, multi_token_support_program.aleo/Token, Final) {
      let transfer_output: (credits.aleo/credits, Final) = credits.aleo/transfer_private_to_public(input_record, self.address, amount);
      let mint_output: (multi_token_support_program.aleo/Token, Final) = multi_token_support_program.aleo/mint_private();
      return (transfer_output.0, mint_output.0, final { transfer_output.1.run(); mint_output.1.run(); });
    }
}
