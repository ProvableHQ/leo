// ANCHOR: file
program external_program.aleo {
    storage counter: u64;

    fn touch() -> Final {
        return final {};
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
