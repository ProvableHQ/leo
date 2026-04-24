---
id: tictactoe
title: A Game of Tic-Tac-Toe in Leo
---

[general tags]: # "example, tictactoe, program, struct, conditional"

**[Source Code](https://github.com/ProvableHQ/leo-examples/tree/main/tictactoe)**

## Summary

We can play a standard game of Tic-Tac-Toe in Leo. I think we all know what a tictactoe board looks like:

⭕ ❕ ⭕ ❕ ❌

➖ ➕ ➖ ➕ ➖

⭕ ❕ ⁣❌ ❕ ⭕

➖ ➕ ➖ ➕ ➖

❌ ❕ ❌ ❕ ⭕

## Representing State

Leo allows users to define composite data types with the `struct` keyword.
The game board is represented by a struct called `Board`, which contains three `Row`s. An alternative representation would be to use an array, however, these are not yet supported in Leo.

## Language Features

- `struct` declarations
- conditional statements
- early termination. Leo allows users to return from a function early using the `return` keyword.

## How to Run

Follow the [Leo Installation Instructions](https://docs.leo-lang.org/getting_started/installation).

This tictactoe program can be run using the following bash script. Locally, it will execute Leo program functions to create and play a game of Tic Tac Toe.

```bash
cd tictactoe
./run.sh
```

## Walkthrough

- [Step 0: Create a new board.](#step0)
- [Step 1: Player 1 makes the first move.](#step1)
- [Step 2: Player 2 makes the second move.](#step2)
- [Step 3: Player 1 makes the third move.](#step3)
- [Step 4: and so on...](#step4)

## <a id="step0"></a> Create a new board

We generate the board, and then the players take turns executing the `make_move` function.

The inputs to the function are the player number, row position, column position, and the previous state of the board.

The output provided is the new state of the board and an evaluation of who won the game. 0u8 as the evaluation output means a draw if the board is complete or that the game is not yet over.

```bash
leo run new
```

## <a id="step1"></a> Player 1 makes a move

Have player 1 make the first move.

```bash
leo run make_move 1u8 1u8 1u8 "{ r1: { c1: 0u8, c2: 0u8, c3: 0u8 }, r2: { c1: 0u8, c2: 0u8, c3: 0u8 }, r3: { c1: 0u8, c2: 0u8, c3: 0u8 } }"
```

## <a id="step2"></a> Player 2 makes a move

Have player 2 make the second move.

```bash
leo run make_move 2u8 2u8 2u8 "{ r1: { c1: 1u8, c2: 0u8, c3: 0u8 }, r2: { c1: 0u8, c2: 0u8, c3: 0u8 }, r3: { c1: 0u8, c2: 0u8, c3: 0u8 } }"
```

## <a id="step3"></a> Player 1 makes another move

Have player 1 make the third move.

```bash
leo run make_move 1u8 3u8 1u8 "{ r1: { c1: 1u8, c2: 0u8, c3: 0u8 }, r2: { c1: 0u8, c2: 2u8, c3: 0u8 }, r3: { c1: 0u8, c2: 0u8, c3: 0u8 } }"
```

## <a id="step4"></a> and so on

If you follow the run script till the end, you'll see the players make a draw, with an output of `0u64`.

This example does not utilize records, as it is primarily designed to demonstrate Leo language features rather than serve as a fully realistic example of a ZK game. A more complete implementation would involve passing records between players to maintain game state securely.
