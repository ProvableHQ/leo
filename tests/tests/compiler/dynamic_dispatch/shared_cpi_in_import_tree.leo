// Test that two programs in the import tree can each call the same external CPI
// function that returns a Final without triggering "Finalization not found".
//
// The bug: when the FromAleo stub (credits.aleo) appears after the FromLeo stub
// (program_b.aleo) in the stubs map, the type checker would try to look up finalize
// types for credits.aleo before they were populated, causing a panic.

// --- aleo stub --- //
program credits.aleo;

record credits:
    owner as address.private;
    amount as u64.private;

function transfer_public_to_private:
    input r0 as address.private;
    input r1 as u64.private;
    cast r0 r1 into r2 as credits.record;
    async transfer_public_to_private into r3;
    output r2 as credits.record;
    output r3 as credits.aleo/transfer_public_to_private.future;

finalize transfer_public_to_private:
    assert.eq 1u8 1u8;

// --- Next Program --- //

import credits.aleo;

program program_b.aleo {
    @noupgrade
    constructor() {}

    fn withdraw(
        owner: address,
        public amount: u64,
    ) -> (credits.aleo::credits, Final) {
        let (c, f): (credits.aleo::credits, Final) =
            credits.aleo::transfer_public_to_private(owner, amount);
        return (c, final {
            f.run();
        });
    }
}

// --- Next Program --- //

import program_b.aleo;
import credits.aleo;

program program_a.aleo {
    @noupgrade
    constructor() {}

    fn payout(
        recipient: address,
        public amount: u64,
    ) -> (credits.aleo::credits, Final) {
        let (c, f): (credits.aleo::credits, Final) =
            credits.aleo::transfer_public_to_private(recipient, amount);
        return (c, final {
            f.run();
        });
    }
}
