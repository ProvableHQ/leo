/*
namespace: Serialize
expectation: Pass
*/

import lib.SillySudoku;

// The `silly-sudoku` main function
function main(puzzle: [u8; (3, 3)], answer: [u8; (3, 3)]) -> bool {
    console.log("Starting Sudoku solver...");
    console.log("{}", puzzle);

    // Instantiate the Sudoku puzzle.
    let sudoku = SillySudoku { puzzle_grid: puzzle };

    console.log("Checking Sudoku answer...");
    console.log("{}", answer);

    // Evaluate the Sudoku puzzle with the given answer.
    let result = sudoku.solve(answer);

    console.log("The answer is {}.", result);

    return result;
}

// Tests that the `silly-sudoku` circuit outputs true on a correct answer.
@test
function test_solve_pass() {
    let puzzle: [u8; (3, 3)] = [[0, 2, 0],
                                [0, 0, 6],
                                [0, 8, 9]];

    let answer: [u8; (3, 3)] = [[1, 2, 3],
                                [4, 5, 6],
                                [7, 8, 9]];

    // Runs the Sudoku checker.
    let result = main(puzzle, answer);

    // Expects the result to be true.
    console.assert(true == result);
}

// Tests that the `silly-sudoku` circuit outputs false on an incorrect answer.
@test
function test_solve_fail() {
    let puzzle: [u8; (3, 3)] = [[0, 2, 0],
                                [0, 0, 6],
                                [0, 8, 0]];

    let answer: [u8; (3, 3)] = [[1, 2, 3],
                                [4, 5, 6],
                                [7, 8, 8]]; // We have an extra `8` in this column!

    // Runs the Sudoku checker.
    let result = main(puzzle, answer);

    // Expects the result to be false.
    console.assert(false == result);
}

// Test that the `silly-sudoku` circuit outputs the expected value on a custom test input.
@test(test_input)
function test_solve_with_input(
    puzzle: [u8; (3, 3)],
    answer: [u8; (3, 3)],
    expected: bool
) {
    // Runs the Sudoku checker.
    let result = main(puzzle, answer);

    console.log("expected {}, got {}", expected, result);

    console.assert(expected == result);
}
