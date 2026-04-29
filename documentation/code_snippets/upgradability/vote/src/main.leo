// ANCHOR: file
import basic_voting.aleo;

// The 'vote_example' program.
program vote_example.aleo {
    // This constructor is for the "checksum" mode.
    @checksum(mapping="basic_voting.aleo::approved_checksum", key="true")
    constructor() {
        // The Leo compiler automatically generates the constructor logic.
    }

    fn main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }
}
// ANCHOR_END: file
