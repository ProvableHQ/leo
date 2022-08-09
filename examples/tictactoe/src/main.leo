// This is suggestive of a tic-tac-toe game.
// There are two players: 1, who uses X; and 2, who uses O.

// The state of the board is captured by a value of this circuit type.
// This will work better with arrays, but for now we just expand the 9 cells.

circuit Board {
    // c(ell){row}{column} (0 for empty, 1 for X, 2 for O):
    c11: u8,
    c12: u8,
    c13: u8,
    c21: u8,
    c22: u8,
    c23: u8,
    c31: u8,
    c32: u8,
    c33: u8
}

// This function checks whether a player is a winner on a board.
// It checks if there are three marks for the player
// in any row, column, or diagonal.

function win(b: Board, p: u8) -> bool {
    return
        (b.c11 == p && b.c12 == p && b.c13 == p) || // row 1
        (b.c21 == p && b.c22 == p && b.c23 == p) || // row 2
        (b.c31 == p && b.c32 == p && b.c33 == p) || // row 3
        (b.c11 == p && b.c21 == p && b.c31 == p) || // column 1
        (b.c12 == p && b.c22 == p && b.c32 == p) || // column 2
        (b.c13 == p && b.c23 == p && b.c33 == p) || // column 3
        (b.c11 == p && b.c22 == p && b.c33 == p) || // diagonal
        (b.c13 == p && b.c22 == p && b.c31 == p); // other diagonal
}

// This carries out a single move: a player (1 for X, 2 for O) marking a cell.
// The cell must be empty (i.e. contain 0), otherwise this is a no-op.
// The u8 result is the winning player, of 0 if there is no winner (yet).

@program
function main(public player: u8, row: u8, col: u8, board: Board) -> (Board, u8) {
    console.assert(player == 1u8 || player == 2u8); // 1 for X, 2 for O
    console.assert(1u8 <= row && row <= 3u8);
    console.assert(1u8 <= col && col <= 3u8);
    let new: Board = board;
    // This will work better when circuit assignment is supported.
    if row == 1u8 && col == 1u8 && new.c11 == 0u8 {
        new = Board {
            c11: player,
            c12: new.c12,
            c13: new.c13,
            c21: new.c21,
            c22: new.c22,
            c23: new.c23,
            c31: new.c31,
            c32: new.c32,
            c33: new.c33,
        };
    } else if row == 1u8 && col == 2u8 && new.c12 == 0u8 {
        new = Board {
            c11: new.c11,
            c12: player,
            c13: new.c13,
            c21: new.c21,
            c22: new.c22,
            c23: new.c23,
            c31: new.c31,
            c32: new.c32,
            c33: new.c33,
        };
    } else if row == 1u8 && col == 3u8 && new.c13 == 0u8 {
        new = Board {
            c11: new.c11,
            c12: new.c12,
            c13: player,
            c21: new.c21,
            c22: new.c22,
            c23: new.c23,
            c31: new.c31,
            c32: new.c32,
            c33: new.c33,
        };
    } else if row == 2u8 && col == 1u8 && new.c21 == 0u8 {
        new = Board {
            c11: new.c11,
            c12: new.c12,
            c13: new.c13,
            c21: player,
            c22: new.c22,
            c23: new.c23,
            c31: new.c31,
            c32: new.c32,
            c33: new.c33,
        };
    } else if row == 2u8 && col == 2u8 && new.c22 == 0u8 {
        new = Board {
            c11: new.c11,
            c12: new.c12,
            c13: new.c13,
            c21: new.c21,
            c22: player,
            c23: new.c23,
            c31: new.c31,
            c32: new.c32,
            c33: new.c33,
        };
    } else if row == 2u8 && col == 3u8 && new.c23 == 0u8 {
        new = Board {
            c11: new.c11,
            c12: new.c12,
            c13: new.c13,
            c21: new.c21,
            c22: new.c22,
            c23: player,
            c31: new.c31,
            c32: new.c32,
            c33: new.c33,
        };
    } else if row == 3u8 && col == 1u8 && new.c31 == 0u8 {
        new = Board {
            c11: new.c11,
            c12: new.c12,
            c13: new.c13,
            c21: new.c21,
            c22: new.c22,
            c23: new.c23,
            c31: player,
            c32: new.c32,
            c33: new.c33,
        };
    } else if row == 3u8 && col == 2u8 && new.c32 == 0u8 {
        new = Board {
            c11: new.c11,
            c12: new.c12,
            c13: new.c13,
            c21: new.c21,
            c22: new.c22,
            c23: new.c23,
            c31: new.c31,
            c32: player,
            c33: new.c33,
        };
    } else if row == 3u8 && col == 3u8 && new.c33 == 0u8 {
        new = Board {
            c11: new.c11,
            c12: new.c12,
            c13: new.c13,
            c21: new.c21,
            c22: new.c22,
            c23: new.c23,
            c31: new.c31,
            c32: new.c32,
            c33: player,
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
