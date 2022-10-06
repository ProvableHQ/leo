/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    struct Row {
        e1: u8,
        e2: u8,
        e3: u8
    }
    
    struct Board {
        r1: Row,
        r2: Row,
        r3: Row,
    }
    
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
    
    transition main(public player: u8, row: u8, col: u8, board: Board) -> (Board, u8) {
        let updated: Board = board;
        if row == 1u8 && col == 1u8 && updated.r1.e1 == 0u8 {
            updated = Board {
                r1: Row {
                    e1: player,
                    e2: updated.r1.e2,
                    e3: updated.r1.e3
                },
                r2: updated.r2,
                r3: updated.r3
            };
        } else if row == 1u8 && col == 2u8 && updated.r1.e2 == 0u8 {
            updated = Board {
                r1: Row {
                    e1: updated.r1.e1,
                    e2: player,
                    e3: updated.r1.e3
                },
                r2: updated.r2,
                r3: updated.r3
            };
        } else if row == 1u8 && col == 3u8 && updated.r1.e3 == 0u8 {
            updated = Board {
                r1: Row {
                    e1: updated.r1.e1,
                    e2: updated.r1.e2,
                    e3: player
                },
                r2: updated.r2,
                r3: updated.r3
            };
        } else if row == 2u8 && col == 1u8 && updated.r2.e1 == 0u8 {
            updated = Board {
                r1: updated.r1,
                r2: Row {
                    e1: player,
                    e2: updated.r2.e2,
                    e3: updated.r2.e3
                },
                r3: updated.r3
            };
        } else if row == 2u8 && col == 2u8 && updated.r2.e2 == 0u8 {
            updated = Board {
                r1: updated.r1,
                r2: Row {
                    e1: updated.r2.e1,
                    e2: player,
                    e3: updated.r2.e3
                },
                r3: updated.r3
            };
        } else if row == 2u8 && col == 3u8 && updated.r2.e3 == 0u8 {
            updated = Board {
                r1: updated.r1,
                r2: Row {
                    e1: updated.r2.e1,
                    e2: updated.r2.e2,
                    e3: player
                },
                r3: updated.r3
            };
        } else if row == 3u8 && col == 1u8 && updated.r3.e1 == 0u8 {
            updated = Board {
                r1: updated.r1,
                r2: updated.r2,
                r3: Row {
                    e1: player,
                    e2: updated.r3.e2,
                    e3: updated.r3.e3
                }
            };
        } else if row == 3u8 && col == 2u8 && updated.r3.e2 == 0u8 {
            updated = Board {
                r1: updated.r1,
                r2: updated.r2,
                r3: Row {
                    e1: updated.r3.e1,
                    e2: player,
                    e3: updated.r3.e3
                }
            };
        } else if row == 3u8 && col == 3u8 && updated.r3.e3 == 0u8 {
            updated = Board {
                r1: updated.r1,
                r2: updated.r2,
                r3: Row {
                    e1: updated.r3.e1,
                    e2: updated.r3.e2,
                    e3: player
                }
            };
        }
        if win(updated, 1u8) {
            return (updated, 1u8);
        } else if win(updated, 2u8) {
            return (updated, 2u8);
        } else {
            return (updated, 0u8);
        }
    }}
