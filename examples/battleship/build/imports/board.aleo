program board.aleo;

record board_state:
    owner as address.private;
    hits_and_misses as u64.private;
    played_tiles as u64.private;
    ships as u64.private;
    player_1 as address.private;
    player_2 as address.private;
    game_started as boolean.private;


function new_board_state:
    input r0 as u64.private;
    input r1 as address.private;
    cast self.caller 0u64 0u64 r0 self.caller r1 false into r2 as board_state.record;
    output r2 as board_state.record;


function start_board:
    input r0 as board_state.record;
    not r0.game_started into r1;
    assert.eq r1 true;
    cast r0.owner r0.hits_and_misses r0.played_tiles r0.ships r0.player_1 r0.player_2 true into r2 as board_state.record;
    output r2 as board_state.record;


function update_played_tiles:
    input r0 as board_state.record;
    input r1 as u64.private;
    sub r1 1u64 into r2;
    and r1 r2 into r3;
    assert.eq r3 0u64;
    and r1 r0.played_tiles into r4;
    assert.eq r4 0u64;
    or r0.played_tiles r1 into r5;
    cast r0.owner r0.hits_and_misses r5 r0.ships r0.player_1 r0.player_2 r0.game_started into r6 as board_state.record;
    output r6 as board_state.record;


function update_hits_and_misses:
    input r0 as board_state.record;
    input r1 as u64.private;
    or r0.hits_and_misses r1 into r2;
    cast r0.owner r2 r0.played_tiles r0.ships r0.player_1 r0.player_2 r0.game_started into r3 as board_state.record;
    output r3 as board_state.record;
