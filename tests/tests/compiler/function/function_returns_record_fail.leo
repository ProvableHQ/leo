/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    record Board {
        owner: address,
        data: u8,
    }

    function foo(board: Board, data: u8) -> Board {
        return Board {
            owner: board.owner,
            data: data,
        };
    }

    function bar(board: Board) {
        assert_eq(board.data, 0u8);
    }

    transition main(board: Board) -> Board {
        bar(board);
        return foo(board, 1u8);
    }



}
