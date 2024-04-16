/*
namespace: Compile
expectation: Pass
*/

// The 'lottery' program.
program lottery.aleo {

    mapping num_winners: u8 => u8;

    record Ticket {
        owner: address,
    }

    async transition play() -> (Ticket, Future) {
        let ticket: Ticket = Ticket {
            owner: self.caller,
        };
        return (ticket, finalize_play());
    }

    async function finalize_play() {
        // Check that the lottery has not expired.
        assert(block.height <= 1000u32);

        // Randomly select whether or not the ticket is a winner.
        assert(ChaCha::rand_bool());

        // Check that the maximum number of winners have not been reached.
        let winners: u8 = num_winners.get_or_use(0u8, 0u8);
        assert(winners < 5u8);
        num_winners.set(0u8, winners + 1u8);

    }
}

