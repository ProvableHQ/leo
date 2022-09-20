circuit Row {
    c1: u8,
    c2: u8,
    c3: u8
}

circuit Board {
    r1: Row,
    r2: Row,
    r3: Row,
}

@program
function new() -> Board {
    return Board {
        r1: Row { c1: 0u8, c2: 0u8, c3: 0u8 },
        r2: Row { c1: 0u8, c2: 0u8, c3: 0u8 },
        r3: Row { c1: 0u8, c2: 0u8, c3: 0u8 },
    };
}

function check_for_win(b: Board, p: u8) -> bool {
    return
        (b.r1.c1 == p && b.r1.c2 == p && b.r1.c3 == p) || // row 1
        (b.r2.c1 == p && b.r2.c2 == p && b.r2.c3 == p) || // row 2
        (b.r3.c1 == p && b.r3.c3 == p && b.r3.c3 == p) || // row 3
        (b.r1.c1 == p && b.r2.c1 == p && b.r3.c1 == p) || // column 1
        (b.r1.c2 == p && b.r2.c3 == p && b.r3.c2 == p) || // column 2
        (b.r1.c3 == p && b.r2.c3 == p && b.r3.c3 == p) || // column 3
        (b.r1.c1 == p && b.r2.c2 == p && b.r3.c3 == p) || // diagonal
        (b.r1.c3 == p && b.r2.c2 == p && b.r3.c1 == p); // other diagonal
}

@program
function make_move(player: u8, row: u8, col: u8, board: Board) -> (Board, u8) {
    console.assert(player == 1u8 || player == 2u8); // 1 for X, 2 for O
    console.assert(1u8 <= row && row <= 3u8);
    console.assert(1u8 <= col && col <= 3u8);

    let r1c1: u8 = board.r1.c1;
    let r1c2: u8 = board.r1.c2;
    let r1c3: u8 = board.r1.c3;
    let r2c1: u8 = board.r2.c1;
    let r2c2: u8 = board.r2.c2;
    let r2c3: u8 = board.r2.c3;
    let r3c1: u8 = board.r3.c1;
    let r3c2: u8 = board.r3.c2;
    let r3c3: u8 = board.r3.c3;

    if row == 1u8 && col == 1u8 && r1c1 == 0u8 {
        r1c1 = player;
    } else if row == 1u8 && col == 2u8 && r1c2 == 0u8 {
        r1c2 = player;
    } else if row == 1u8 && col == 3u8 && r1c3 == 0u8 {
        r1c3 = player;
    } else if row == 2u8 && col == 1u8 && r2c1 == 0u8 {
        r2c1 = player;
    } else if row == 2u8 && col == 2u8 && r2c2 == 0u8 {
        r2c2 = player;
    } else if row == 2u8 && col == 3u8 && r2c3 == 0u8 {
        r2c3 = player;
    } else if row == 3u8 && col == 1u8 && r3c1 == 0u8 {
        r3c1 = player;
    } else if row == 3u8 && col == 2u8 && r3c2 == 0u8 {
        r3c2 = player;
    } else if row == 3u8 && col == 3u8 && r3c3 == 0u8 {
        r3c3 = player;
    }

    let updated: Board = Board {
        r1: Row { c1: r1c1, c2: r1c2, c3: r1c3 },
        r2: Row { c1: r2c1, c2: r2c2, c3: r2c3 },
        r3: Row { c1: r3c1, c2: r2c2, c3: r2c3 },
    };

    if check_for_win(updated, 1u8) {
        return (updated, 1u8);
    } else if check_for_win(updated, 2u8) {
        return (updated, 2u8);
    } else {
        return (updated, 0u8);
    }
}
