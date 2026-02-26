program provider.aleo {
    storage x: u32;

    fn init() -> Final {
        return final {};
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import provider.aleo;

program user.aleo {
    fn try_modify() -> Final {
        return final {
            provider.aleo/x = 0;         // ❌ illegal external mutation
        };
    }

    @noupgrade
    constructor() {}
}
