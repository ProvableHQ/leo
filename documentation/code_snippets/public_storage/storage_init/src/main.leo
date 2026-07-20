// ANCHOR: file
program storage_init.aleo {
    storage counter: u32;

    fn read() -> Final {
        return final {
            let c: u32 = counter.unwrap();
        };
    }

    @custom
    constructor() {
        // Only initialize on the first deployment (edition 0). On upgrade
        // (edition > 0) the existing storage is preserved.
        if std::ctx::edition() == 0u16 {
            counter = 0u32;
        }
    }
}
// ANCHOR_END: file
