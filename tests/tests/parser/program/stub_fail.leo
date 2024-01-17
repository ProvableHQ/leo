/*
namespace: Parse
expectation: Fail
*/

stub credits.aleo {
    import battleship.aleo;
    import exchange.aleo;
    record credits {
        owner: address,
        micrcredits: u64,
    }
    struct Row {
        c1: u8,
        c2: u8,
        c3: u8
    }
    mapping num_winners: u8 => u8;

    record board_state {
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

    transition mint(public a1: address, public a2: u64) -> credits;

    transition transfer_public(public a1: address, public a2: u64) -> exchange.aleo/tokens;

    transition transfer_private(a1: credits, a2: address, a3: u64) -> (credits, credits);

    transition transfer_private_to_public(a1: credits, public a2: address, public a3: u64) -> credits;

    transition transfer_public_to_private(public a1: address, public a2: u64) -> credits;

    transition join(a1: credits, a2: credits) -> credits;

    transition split(a1: credits, a2: u64) -> (credits, credits);

    function foo() {
       let a:u8 = 1u8;
    }
    finalize foo() {
        let c:u8 = 1u8;
    }
}

program test.aleo {
    transition foo(arg1: address, arg1: u64, cred: credits.aleo/credits)->credits.aleo/credits {
        credits.aleo/transfer_public(arg1, arg2);
        credits.leo/transfer_public(arg1, arg2);
    }
}