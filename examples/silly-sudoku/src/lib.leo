/**
 * The SillySudoku circuit
 *
 * This circuit generates a silly Sudoku puzzle,
 * by constructing a 3x3 puzzle grid with some preset numbers 1-9,
 * and requiring an answer where each number is used exactly once.
 *
 *  -----------
 * | 5 | 8 | 3 |
 * |-----------|
 * | 2 | 7 | 4 |
 * |-----------|
 * | 1 | 9 | 6 |
 *  -----------
 */
circuit SillySudoku {
    // The starting grid values for the Sudoku puzzle.
    // Unset cells on the puzzle grid are set to 0.
    puzzle_grid: [u8; (3, 3)];

    /**
     * Returns true if a given Sudoku answer is correct.
     * 
     * Verifies a given answer by iterating through the Sudoku puzzle,
     * and checking that each number is set exactly once.
     */
    function solve(self, answer: [u8; (3, 3)]) -> bool {
        // The result boolean is set to true, if the answer is correct.
        let result = true;
        // An array that tracks the numbers used on the Sudoku grid.
        let seen = [false; 9];
        
        // Iterate through the Sudoku grid and check each cell.
        for i in 0..3 {
            for j in 0..3 {
                
                // Fetch the current cell value for the Sudoku grid.
                let grid_value = self.puzzle_grid[i][j];

                // Fetch the current cell value for the given answer.
                let answer_value = answer[i][j];

                // Set the index by subtracting 1 from the answer value.
                let index = answer_value - 1;

                // Check if this number has already been used on the grid.
                let already_seen: bool = seen[index];

                // If this number is already used, the answer is incorrect.
                // Sets the result to false.
                if already_seen {
                    result = false;
                }

                // If the cell is not empty, and the grid value doesn't match
                // the answer value, the answer is incorrect.
                // Sets the result to false.
                if (grid_value != 0 && grid_value != answer_value) {
                    result = false;
                }

                // Sets the answer value as seen.
                seen[index] = true;
            }
        }

        // Returns true if all numbers 1-9 have been seen exactly once.
        return result;
    }
}
