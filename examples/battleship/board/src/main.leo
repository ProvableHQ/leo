program board.aleo {
    // Battleship boards are represented by 8x8 squares.
    // A u64 is all that is required to represent a hit or a miss on a single board.
    // Starting from the top row, left to right, a hit is 1 and a miss is 0.
    // A first move resulting in a hit in row 1, column 3 would be:
    // 00100000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
    // A second u64 is needed to represent which squares have been played, with 1s being played squares and 0s being
    // unplayed squares.
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

    // Returns a new board_state.
    transition new_board_state(
        ships: u64,
        opponent: address,
    ) -> board_state {
        return board_state {
            owner: self.caller,
            hits_and_misses: 0u64,
            played_tiles: 0u64,
            ships,
            player_1: self.caller,
            player_2: opponent,
            game_started: false,
        };
    }

    // Returns a new board state that has been started.
    // Fails if this board has been started before.
    transition start_board(
        // The record of the board to start. A board can only be started once.
        board: board_state,
    ) -> board_state {
        // Ensure this board hasn't been used to start a game before.
        assert(!board.game_started);

        return board_state {
            owner: board.owner,
            hits_and_misses: board.hits_and_misses,
            played_tiles: board.played_tiles,
            ships: board.ships,
            player_1: board.player_1,
            player_2: board.player_2,
            game_started: true,
        };
    }

    // Returns a new board state record that includes all the played tiles.
    // Fails if r1 has been played before.
    transition update_played_tiles(
        // The record of the board to update.
        board: board_state,
        // The u64 equivalent of a bitstring fire coordinate to send to the opponent.
        shoot: u64,
    ) -> board_state {
        // Need to make sure r1 is a valid move. Only one bit of r1 should be flipped.
        let flip_bit: u64 = shoot - 1u64;
        // bitwise and operation
        let check_move: u64 = shoot & flip_bit;
        assert_eq(check_move, 0u64);

        // Need to make sure r1 is a valid move given the played_tiles. no bits should overlap.
        let check_tiles: u64 = shoot & board.played_tiles;
        assert_eq(check_tiles, 0u64);

        // Update played tiles.
        let played_tiles: u64 = board.played_tiles | shoot;

        return board_state {
            owner: board.owner,
            hits_and_misses: board.hits_and_misses,
            played_tiles,
            ships: board.ships,
            player_1: board.player_1,
            player_2: board.player_2,
            game_started: board.game_started,
        };
    }

    // Returns a new board state record that includes all the hits and misses.
    transition update_hits_and_misses(
        // The record of the board to update.
        board: board_state,
        // The u64 equivalent of a bitstring of whether this player's previous move was a hit or miss.
        hit_or_miss: u64,
    ) -> board_state {
        // Update hits and misses.
        let hits_and_misses: u64 = board.hits_and_misses | hit_or_miss;

        return board_state {
            owner: board.owner,
            hits_and_misses,
            played_tiles: board.played_tiles,
            ships: board.ships,
            player_1: board.player_1,
            player_2: board.player_2,
            game_started: board.game_started,
        };
    }
}
