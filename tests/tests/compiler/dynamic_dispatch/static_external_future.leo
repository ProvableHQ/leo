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
}

final fn finalize() {
    assert_eq(1u8, 1u8);
}

// --- Next Program --- //

import credits.aleo;

program test_static.aleo {
    @noupgrade
    constructor() {}

    fn test_withdraw_private(amount: u64) -> (credits.aleo::credits, Final) {
        let (cred, f): (credits.aleo::credits, Final) = credits.aleo::transfer_public_to_private(self.signer, amount);
        return (cred, final { f.run(); });
    }
}
