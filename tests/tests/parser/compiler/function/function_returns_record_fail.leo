/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    record Board {
        owner: address,
        gates: u64,
        data: u8,
    }

    function foo(board: Board, data: u8) -> Board {
        return Board {
            owner: board.owner,
            gates: board.gates,
            data: data,
        };
    }

    function bar(board: Board) {
        assert_eq(board.gates, 0u64);
    }

    transition main(board: Board) -> Board {
        bar(board);
        return foo(board, 1u8);
    }



}
