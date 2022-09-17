// This is suggestive of a tic-tac-toe game.
// There are two players: 1, who uses X; and 2, who uses O.

// The state of the board is captured by a value of this circuit type.
// This will work better with arrays, but for now we just expand the 9 cells.

circuit Row {
    e1: u8,
    e2: u8,
    e3: u8
}

circuit Board {
    r1: Row,
    r2: Row,
    r3: Row,
}

// This function creates a new game board, with all cells empty.
@program
function new() -> Board {
    return Board {
        r1: Row { e1: 0u8, e2: 0u8, e3: 0u8 },
        r2: Row { e1: 0u8, e2: 0u8, e3: 0u8 },
        r3: Row { e1: 0u8, e2: 0u8, e3: 0u8 },
    };
}

// This function checks whether a player is a winner on a board.
// It checks if there are three marks for the player
// in any row, column, or diagonal.

function win(b: Board, p: u8) -> bool {
    return
        (b.r1.e1 == p && b.r1.e2 == p && b.r1.e3 == p) || // row 1
        (b.r2.e1 == p && b.r2.e2 == p && b.r2.e3 == p) || // row 2
        (b.r3.e1 == p && b.r3.e3 == p && b.r3.e3 == p) || // row 3
        (b.r1.e1 == p && b.r2.e1 == p && b.r3.e1 == p) || // column 1
        (b.r1.e2 == p && b.r2.e3 == p && b.r3.e2 == p) || // column 2
        (b.r1.e3 == p && b.r2.e3 == p && b.r3.e3 == p) || // column 3
        (b.r1.e1 == p && b.r2.e2 == p && b.r3.e3 == p) || // diagonal
        (b.r1.e3 == p && b.r2.e2 == p && b.r3.e1 == p); // other diagonal
}

// This carries out a single move: a player (1 for X, 2 for O) marking a cell.
// The cell must be empty (i.e. contain 0), otherwise this is a no-op.
// The u8 result is the winning player, of 0 if there is no winner (yet).
//
@program
function main(public player: u8, row: u8, col: u8, board: Board) -> (Board, u8) {
    console.assert(player == 1u8 || player == 2u8); // 1 for X, 2 for O
    console.assert(1u8 <= row && row <= 3u8);
    console.assert(1u8 <= col && col <= 3u8);
    let new: Board = board;
    // This will work better when circuit assignment is supported.
    if row == 1u8 && col == 1u8 && new.r1.e1 == 0u8 {
        new = Board {
            r1: Row {
                e1: player,
                e2: new.r1.e2,
                e3: new.r1.e3
            },
            r2: new.r2,
            r3: new.r3
        };
    } else if row == 1u8 && col == 2u8 && new.r1.e2 == 0u8 {
        new = Board {
            r1: Row {
                e1: new.r1.e1,
                e2: player,
                e3: new.r1.e3
            },
            r2: new.r2,
            r3: new.r3
        };
    } else if row == 1u8 && col == 3u8 && new.r1.e3 == 0u8 {
        new = Board {
            r1: Row {
                e1: new.r1.e1,
                e2: new.r1.e2,
                e3: player
            },
            r2: new.r2,
            r3: new.r3
        };
    } else if row == 2u8 && col == 1u8 && new.r2.e1 == 0u8 {
        new = Board {
            r1: new.r1,
            r2: Row {
                e1: player,
                e2: new.r2.e2,
                e3: new.r2.e3
            },
            r3: new.r3
        };
    } else if row == 2u8 && col == 2u8 && new.r2.e2 == 0u8 {
        new = Board {
            r1: new.r1,
            r2: Row {
                e1: new.r2.e1,
                e2: player,
                e3: new.r2.e3
            },
            r3: new.r3
        };
    } else if row == 2u8 && col == 3u8 && new.r2.e3 == 0u8 {
        new = Board {
            r1: new.r1,
            r2: Row {
                e1: new.r2.e1,
                e2: new.r2.e2,
                e3: player
            },
            r3: new.r3
        };
    } else if row == 3u8 && col == 1u8 && new.r3.e1 == 0u8 {
        new = Board {
            r1: new.r1,
            r2: new.r2,
            r3: Row {
                e1: player,
                e2: new.r3.e2,
                e3: new.r3.e3
            }
        };
    } else if row == 3u8 && col == 2u8 && new.r3.e2 == 0u8 {
        new = Board {
            r1: new.r1,
            r2: new.r2,
            r3: Row {
                e1: new.r3.e1,
                e2: player,
                e3: new.r3.e3
            }
        };
    } else if row == 3u8 && col == 3u8 && new.r3.e3 == 0u8 {
        new = Board {
            r1: new.r1,
            r2: new.r2,
            r3: Row {
                e1: new.r3.e1,
                e2: new.r3.e2,
                e3: player
            }
        };
    }
    if win(new, 1u8) {
        return (new, 1u8);
    } else if win(new, 2u8) {
        return (new, 2u8);
    } else {
        return (new, 0u8);
    }
}
