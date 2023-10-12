/*
namespace: Compile
expectation: Fail
*/

stub credits.aleo {
    record credits {
        owner: address,
        microcredits: u64,
    }
    mapping num_winners: u8 => u8;
    mapping num_tx: address => u8;
    mapping neighbor: address => credits;

    const BIG: u64 = 1_000_000u64;
    const SMALL: u128 = 0u128;

    struct board_state {
        owner: address,
        // The hits and misses registered on the opponent's board.
        hits_and_misses: u64,
        // The squares that have been played on the opponent's board.
        played_tiles: u64,
        // The ship bitstring representing all ship positions on your own board
        ships: u64,
        player_1: address,
        player_2: address,
        game_started: bool,
    }
}

program test.aleo {
    transition main(arg1: address, arg2: u64, arg3: credits.aleo/credits) -> credits.aleo/credits {
        // External call
        credits.aleo/transfer_public(arg1, arg2);

        return mint(arg1, arg2);
    }
}