import credits.aleo;

program credits_reader.aleo {
    fn read(addr: address) -> Final {
        return final {
            // ANCHOR: cross_program_reads
            let balance1 = credits.aleo::account.get(addr);
            let balance2 = credits.aleo::account.get_or_use(addr, 0u64);
            // ANCHOR_END: cross_program_reads
        };
    }

    @noupgrade
    constructor() {}
}
