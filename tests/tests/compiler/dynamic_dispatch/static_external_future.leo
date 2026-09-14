export struct Metadata {
    recipient: address,
    amount: u64,
}

program token_registry.aleo {
    record Token {
        owner: address,
        amount: u128,
        metadata: field,
    }

    record Permission {
        owner: address,
        allowed: bool,
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

    fn local_record_output(
        input: token_registry.aleo::Token,
        amount: u64,
    ) -> (token_registry.aleo::Token, LocalReceipt) {
        let receipt: LocalReceipt = LocalReceipt { owner: input.owner, amount };
        return (input, receipt);
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

    fn multiple_inputs_local_output(
        first: token_registry.aleo::Token,
        second: token_registry.aleo::Token,
    ) -> (token_registry.aleo::Token, token_registry.aleo::Token, LocalReceipt) {
        let receipt: LocalReceipt = LocalReceipt {
            owner: first.owner,
            amount: (first.amount + second.amount) as u64,
        };
        return (first, second, receipt);
    }

    fn mixed_record_origins(
        token: token_registry.aleo::Token,
        cred: credits.aleo::credits,
    ) -> (token_registry.aleo::Token, credits.aleo::credits, LocalReceipt) {
        let receipt: LocalReceipt = LocalReceipt { owner: cred.owner, amount: token.amount as u64 };
        return (token, cred, receipt);
    }

    fn same_program_records(
        token: token_registry.aleo::Token,
        permission: token_registry.aleo::Permission,
    ) -> (token_registry.aleo::Token, token_registry.aleo::Permission) {
        return (token, permission);
    }

    fn permission_local_output(
        permission: token_registry.aleo::Permission,
    ) -> (token_registry.aleo::Permission, LocalReceipt) {
        let receipt: LocalReceipt = LocalReceipt { owner: permission.owner, amount: 0u64 };
        return (permission, receipt);
    }

    fn local_input_external_output(input: LocalReceipt) -> (credits.aleo::credits, Final) {
        let (cred, credits_future): (credits.aleo::credits, Final) =
            credits.aleo::transfer_public_to_private(input.owner, input.amount);
        return (cred, final {
            credits_future.run();
        });
    }

    fn external_struct_input(input: token_registry.aleo::Metadata) -> LocalReceipt {
        return LocalReceipt { owner: input.recipient, amount: input.amount };
    }

    fn external_struct_output(input: credits.aleo::credits) -> token_registry.aleo::Metadata {
        return token_registry.aleo::Metadata { recipient: input.owner, amount: 0u64 };
    }

    fn mixed_dynamic_origins(
        input: token_registry.aleo::Token,
        dynamic: dyn record,
        receipt: LocalReceipt,
    ) -> (token_registry.aleo::Token, dyn record, LocalReceipt) {
        return (input, dynamic, receipt);
    }

    fn dynamic_input_concrete_output(input: dyn record, receipt: LocalReceipt) -> (dyn record, LocalReceipt) {
        return (input, receipt);
    }

    fn record_input_scalar_output(input: token_registry.aleo::Token) -> u128 {
        return input.amount;
    }
}
