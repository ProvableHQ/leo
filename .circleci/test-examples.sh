# Build and run the auction Leo program.
(
  cd ./project/examples/auction || exit
  $LEO run main
)

# Build and run the bubblesort Leo program.
(
  cd ./project/examples/bubblesort || exit
  $LEO run bubblesort
)

# Build and run the core example Leo program.
(
  cd ./project/examples/core || exit
  $LEO run main
)

# Build and run the groups example Leo program.
(
  cd ./project/examples/groups || exit
  $LEO run main
)

# Build and run the helloworld Leo program.
(
  cd ./project/examples/helloworld || exit
  $LEO run main
)

# Build and run the import point example Leo program.
(
  cd ./project/examples/import_point || exit
  $LEO run main
)

# Build and run the interest example Leo programs.
(
  cd ./project/examples/import_point || exit

  # Run the fixed period interest program.
  $LEO run fixed_period_interest

  # Run the bounded period interest program.
  $LEO run bounded_period_interest
)

# Build and run the message example Leo program.
(
  cd ./project/examples/message || exit
  $LEO run main
)

# Build and run the tic tac toe example Leo program.
(
  cd ./project/examples/tictactoe || exit
  $LEO run main
)

# Build and run the token example programs.
(
  cd ./project/examples/token || exit

  # Run the mint program.
  $LEO run mint

  # Run the transfer program.
  $LEO run transfer
)