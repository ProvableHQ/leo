program move.aleo {
    record move {
        owner: address,
        incoming_fire_coordinate: u64,
        player_1: address,
        player_2: address,
        // One flipped bit indicates a hit. No flipped bits indicates a miss.
        prev_hit_or_miss: u64,
    }

    // Returns new move record owned by the opponent.
    transition create_move(
        // The move record created by the opponent.
        move_record: move,
        // The u64 representation of incoming_fire_coordinate, the bitstring fire coordinate to send to the opponent.
        incoming_fire_coordinate: u64,
        // The u64 representation of prev_hit_or_miss, this player's previous fire coordinate as a hit or miss.
        prev_hit_or_miss: u64,
    ) -> move {
        // A new move record should be created and owned by the opponent.
        let one_is_owner: bool = move_record.player_1 == move_record.owner;
        let opponent: address = one_is_owner ? move_record.player_2 : move_record.player_1;

        return move {
            owner: opponent,
            incoming_fire_coordinate,
            player_1: move_record.player_2,
            player_2: move_record.player_1,
            prev_hit_or_miss,
        };
    }

    // Returns the move record owned by the opponent.
    // Note, this move record contains dummy fire coordinates and previous hit or miss.
    transition start_game(player_2: address) -> move {
        return move {
            owner: player_2,
            incoming_fire_coordinate: 0u64,
            player_1: self.caller,
            player_2: player_2,
            prev_hit_or_miss: 0u64,
        };
    }
}


