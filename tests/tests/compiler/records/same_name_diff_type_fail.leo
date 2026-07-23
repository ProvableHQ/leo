program child.aleo {
    record R {
        owner: address,
        x: u8,
    }

    fn create() -> R {
        return R { owner: std::ctx::signer(), x: 1u8 };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import child.aleo;

program parent.aleo {
    record R {
        owner: address,
        x: u16,
    }

    fn check() -> bool {
        let r1: R = R { owner: std::ctx::signer(), x: 1u16 };
        return r1 == child.aleo::create();
    }

    @noupgrade
    constructor() {}
}
