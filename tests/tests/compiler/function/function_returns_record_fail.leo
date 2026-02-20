
fn foo(board: Board, data: u8) -> Board {
    return Board {
        owner: board.owner,
        data: data,
    };
}

fn bar(board: Board) {
    assert_eq(board.data, 0u8);
}

program test.aleo {
    record Board {
        owner: address,
        data: u8,
    }

    fn main(board: Board) -> Board {
        bar(board);
        return foo(board, 1u8);
    }



}
