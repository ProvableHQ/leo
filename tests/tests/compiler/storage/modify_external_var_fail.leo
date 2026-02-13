program provider.aleo {
    storage x: u32;

    async transition init() -> Future {
        return async {};
    }

    @noupgrade
    async constructor() {}
}

// --- Next Program --- //

import provider.aleo;

program user.aleo {
    async transition try_modify() -> Future {
        return async {
            provider.aleo/x = 0;         // ❌ illegal external mutation
        };
    }

    @noupgrade
    async constructor() {}
}
