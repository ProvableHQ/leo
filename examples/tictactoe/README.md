<!-- # ⭕ Tic-Tac-Toe -->
<img alt="workshop/tictactoe" width="1412" src="../.resources/tictactoe.png">

A standard game of Tic-Tac-Toe in Leo.

⭕ ❕ ⭕ ❕ ❌

➖ ➕ ➖ ➕ ➖

⭕ ❕ ⁣❌ ❕ ⭕

➖ ➕ ➖ ➕ ➖

❌ ❕ ❌ ❕ ⭕

## Representing State
Leo allows users to define composite data types with the `circuit` keyword. 
The game board is represented by a circuit called `Board`, which contains three `Row`s.
An alternative representation would be to use an array, however, these are not yet supported in Leo.

## Language Features
- `circuit` declarations
- conditional statements
- early termination. Leo allows users to return from a function early using the `return` keyword.

## Running the Program

Leo provides users with a command line interface for compiling and running Leo programs.
Users may either specify input values via the command line or provide an input file in `inputs/`.

### Providing inputs via the command line.
1. Run 
```bash
leo run <function_name> <input_1> <input_2> ...
```
See `./run.sh` for an example.


### Using an input file.
1. Modify `inputs/tictactoe.in` with the desired inputs.
2. Run
```bash
leo run <function_name>
```
