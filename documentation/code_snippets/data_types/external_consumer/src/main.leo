import external_program.aleo;

program consumer.aleo {
    fn build() -> external_program.aleo::S2 {
        // ANCHOR: file
        let s: external_program.aleo::S2 = external_program.aleo::S2 {
            x: 1field,
            y: 2u32,
        };
        // ANCHOR_END: file
        return s;
    }

    @noupgrade
    constructor() {}
}
