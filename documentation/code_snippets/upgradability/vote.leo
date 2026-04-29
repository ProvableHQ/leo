program basic_voting.aleo {
    mapping approved_checksum: bool => [u8; 32];

    fn approve(value: [u8; 32]) -> Final {
        return final {
            Mapping::set(approved_checksum, true, value);
        };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
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
