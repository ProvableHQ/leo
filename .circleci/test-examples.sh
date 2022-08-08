# Build and run the helloworld Leo program.
(
  cd ./project/examples/helloworld || exit
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

# Build and run the import point example Leo program.
(
  cd ./project/examples/import_point || exit
  $LEO run main
)

# Build and run the message example Leo program.
(
  cd ./project/examples/message || exit
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