program token_registry.aleo {
    record Token {
        owner: address,
        amount: u128,
        metadata: field,
    }

    fn burn_private(input: Token, amount: u128) -> (Token, Final) {
        let change: Token = Token {
            owner: input.owner,
            amount: input.amount - amount,
            metadata: input.metadata,
        };
        let f: Final = final { finalize(); };
        return (change, f);
    }

    fn consume_private(input: Token) -> u64 {
        return input.amount as u64;
    }

    fn refresh(input: Token) -> Token {
        return input;
    }

    @noupgrade
    constructor() {}
}

export final fn finalize() {
    assert_eq(1u8, 1u8);
}

// --- Next Program --- //

program credits.aleo {
    record credits {
        owner: address,
        amount: u64,
    }

    fn transfer_public_to_private(receiver: address, amount: u64) -> (credits, Final) {
        let cred: credits = credits { owner: receiver, amount };
        let f: Final = final { finalize(); };
        return (cred, f);
    }

    @noupgrade
    constructor() {}
}

export final fn finalize() {
    assert_eq(1u8, 1u8);
}

// --- Next Program --- //

import credits.aleo;
import token_registry.aleo;

program test_static.aleo {
    record LocalReceipt {
        owner: address,
        amount: u64,
    }

    @noupgrade
    constructor() {}

    fn test_withdraw_private(amount: u64) -> (credits.aleo::credits, Final) {
        let (cred, f): (credits.aleo::credits, Final) = credits.aleo::transfer_public_to_private(std::ctx::signer(), amount);
        return (cred, final { f.run(); });
    }

    fn direct_record_conversion(input: token_registry.aleo::Token, amount: u64) -> (token_registry.aleo::Token, credits.aleo::credits, Final) {
        let (cred, credits_future): (credits.aleo::credits, Final) =
            credits.aleo::transfer_public_to_private(input.owner, amount);
        let (change, burn_future): (token_registry.aleo::Token, Final) =
            token_registry.aleo::burn_private(input, amount as u128);
        return (change, cred, final {
            credits_future.run();
            burn_future.run();
        });
    }

    fn asserted_record_conversion(input: token_registry.aleo::Token, amount: u64) -> (token_registry.aleo::Token, credits.aleo::credits, Final) {
        assert_eq(input.metadata, 0field);
        let (cred, credits_future): (credits.aleo::credits, Final) =
            credits.aleo::transfer_public_to_private(input.owner, amount);
        let (change, burn_future): (token_registry.aleo::Token, Final) =
            token_registry.aleo::burn_private(input, amount as u128);
        return (change, cred, final {
            credits_future.run();
            burn_future.run();
        });
    }

    fn conditional_assert_conversion(
        input: token_registry.aleo::Token,
        amount: u64,
        should_check: bool,
    ) -> (token_registry.aleo::Token, credits.aleo::credits, Final) {
        if should_check {
            assert_eq(input.metadata, 0field);
        }
        let (cred, credits_future): (credits.aleo::credits, Final) =
            credits.aleo::transfer_public_to_private(input.owner, amount);
        let (change, burn_future): (token_registry.aleo::Token, Final) =
            token_registry.aleo::burn_private(input, amount as u128);
        return (change, cred, final {
            credits_future.run();
            burn_future.run();
        });
    }

    fn aliased_record_conversion(input: token_registry.aleo::Token, amount: u64) -> (token_registry.aleo::Token, credits.aleo::credits, Final) {
        let alias: token_registry.aleo::Token = input;
        let (cred, credits_future): (credits.aleo::credits, Final) =
            credits.aleo::transfer_public_to_private(alias.owner, amount);
        let (change, burn_future): (token_registry.aleo::Token, Final) =
            token_registry.aleo::burn_private(alias, amount as u128);
        return (change, cred, final {
            credits_future.run();
            burn_future.run();
        });
    }

    fn loop_assert_record_conversion(input: token_registry.aleo::Token, amount: u64) -> (token_registry.aleo::Token, credits.aleo::credits, Final) {
        for i: u8 in 0u8..0u8 {
            assert_eq(input.metadata, 0field);
        }
        let (cred, credits_future): (credits.aleo::credits, Final) =
            credits.aleo::transfer_public_to_private(input.owner, amount);
        let (change, burn_future): (token_registry.aleo::Token, Final) =
            token_registry.aleo::burn_private(input, amount as u128);
        return (change, cred, final {
            credits_future.run();
            burn_future.run();
        });
    }

    fn reassigned_record_conversion(
        input: token_registry.aleo::Token,
        replacement: token_registry.aleo::Token,
        amount: u64,
    ) -> (token_registry.aleo::Token, credits.aleo::credits, Final) {
        assert_eq(input.metadata, 0field);
        input = replacement;
        let (cred, credits_future): (credits.aleo::credits, Final) =
            credits.aleo::transfer_public_to_private(input.owner, amount);
        let (change, burn_future): (token_registry.aleo::Token, Final) =
            token_registry.aleo::burn_private(input, amount as u128);
        return (change, cred, final {
            credits_future.run();
            burn_future.run();
        });
    }

    fn const_assert_record_conversion(input: token_registry.aleo::Token, amount: u64) -> (token_registry.aleo::Token, credits.aleo::credits, Final) {
        const EXPECTED_METADATA: field = 0field;
        assert_eq(input.metadata, EXPECTED_METADATA);
        let (cred, credits_future): (credits.aleo::credits, Final) =
            credits.aleo::transfer_public_to_private(input.owner, amount);
        let (change, burn_future): (token_registry.aleo::Token, Final) =
            token_registry.aleo::burn_private(input, amount as u128);
        return (change, cred, final {
            credits_future.run();
            burn_future.run();
        });
    }

    fn dynamic_record_output(
        input: token_registry.aleo::Token,
        amount: u64,
        program_id: field,
        network_id: field,
        function_id: field,
    ) -> (token_registry.aleo::Token, dyn record, Final) {
        let released: dyn record = _dynamic_call::[dyn record](program_id, network_id, function_id, amount);
        let (change, burn_future): (token_registry.aleo::Token, Final) =
            token_registry.aleo::burn_private(input, amount as u128);
        return (change, released, final {
            burn_future.run();
        });
    }

    fn tuple_alias_record_conversion(input: token_registry.aleo::Token, amount: u64) -> (token_registry.aleo::Token, credits.aleo::credits, Final) {
        let (alias, copied_amount): (token_registry.aleo::Token, u64) = (input, amount);
        let (cred, credits_future): (credits.aleo::credits, Final) =
            credits.aleo::transfer_public_to_private(alias.owner, copied_amount);
        let (change, burn_future): (token_registry.aleo::Token, Final) =
            token_registry.aleo::burn_private(alias, copied_amount as u128);
        return (change, cred, final {
            credits_future.run();
            burn_future.run();
        });
    }

    fn nested_record_conversion(input: token_registry.aleo::Token) -> (credits.aleo::credits, Final) {
        let (cred, credits_future): (credits.aleo::credits, Final) = credits.aleo::transfer_public_to_private(
            input.owner,
            token_registry.aleo::consume_private(input),
        );
        return (cred, final {
            credits_future.run();
        });
    }

    fn expr_reassign_record_conversion(
        input: token_registry.aleo::Token,
        replacement: token_registry.aleo::Token,
        amount: u64,
    ) -> (token_registry.aleo::Token, credits.aleo::credits, Final) {
        assert_eq(input.metadata, 0field);
        input = (replacement, amount).0;
        let (cred, credits_future): (credits.aleo::credits, Final) =
            credits.aleo::transfer_public_to_private(input.owner, amount);
        let (change, burn_future): (token_registry.aleo::Token, Final) =
            token_registry.aleo::burn_private(input, amount as u128);
        return (change, cred, final {
            credits_future.run();
            burn_future.run();
        });
    }

    fn reassign_call_conversion(
        input: token_registry.aleo::Token,
        amount: u64,
    ) -> (token_registry.aleo::Token, credits.aleo::credits, Final) {
        input = token_registry.aleo::refresh(input);
        let (cred, credits_future): (credits.aleo::credits, Final) =
            credits.aleo::transfer_public_to_private(input.owner, amount);
        return (input, cred, final {
            credits_future.run();
        });
    }

    fn tuple_project_record_conversion(input: token_registry.aleo::Token) -> (credits.aleo::credits, Final) {
        let pair: (token_registry.aleo::Token, u64) = (input, 0u64);
        let amount: u64 = token_registry.aleo::consume_private(pair.0);
        let (cred, credits_future): (credits.aleo::credits, Final) =
            credits.aleo::transfer_public_to_private(input.owner, amount);
        return (cred, final {
            credits_future.run();
        });
    }

    fn intrinsic_record_conversion(input: token_registry.aleo::Token) -> (credits.aleo::credits, Final) {
        let amount: u64 = _sha3_512_hash_to_u64(token_registry.aleo::consume_private(input));
        let (cred, credits_future): (credits.aleo::credits, Final) =
            credits.aleo::transfer_public_to_private(input.owner, amount);
        return (cred, final {
            credits_future.run();
        });
    }

    fn local_record_output(
        input: token_registry.aleo::Token,
        amount: u64,
    ) -> (token_registry.aleo::Token, LocalReceipt, Final) {
        let receipt: LocalReceipt = LocalReceipt { owner: input.owner, amount };
        let (change, burn_future): (token_registry.aleo::Token, Final) =
            token_registry.aleo::burn_private(input, amount as u128);
        return (change, receipt, final {
            burn_future.run();
        });
    }

    fn external_output_no_consumption(
        input: token_registry.aleo::Token,
        amount: u64,
    ) -> (token_registry.aleo::Token, credits.aleo::credits, Final) {
        let (cred, credits_future): (credits.aleo::credits, Final) =
            credits.aleo::transfer_public_to_private(input.owner, amount);
        return (input, cred, final {
            credits_future.run();
        });
    }
}
