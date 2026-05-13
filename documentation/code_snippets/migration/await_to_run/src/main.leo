// ANCHOR: file
// 4.0
import other_program.aleo;

program example.aleo {
    fn compose(value: u8) -> Final {
        let f: Final = other_program.aleo::action();
        return final {
            f.run();
            // ... on-chain logic
        };
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
