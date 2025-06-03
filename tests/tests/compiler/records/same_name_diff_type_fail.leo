
program child.aleo {
    record R {
        owner: address,
        x: u8,
    }

    transition create() -> R {
        return R {
            owner: self.signer,
            x: 1u8,
        };
    }
}

// --- Next Program --- //

import child.aleo;
program parent.aleo {
    record R {
        owner: address,
        x: u16,
    }

    transition check() -> bool {
        let r1: R = R {
            owner: self.signer,
            x: 1u16,
        };

        return r1 == child.aleo/create();
    }
}
