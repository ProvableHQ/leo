# Build and run the auction Leo program.
(
  cd ./project/examples/auction || exit
  $LEO run place_bid
  $LEO run resolve
  $LEO run finish

  chmod +x ./run.sh
  ./run.sh
)

# Build and run the broken_bank Leo program.
(
  cd ./project/examples/broken_bank || exit
  $LEO run issue
  $LEO run deposit
  $LEO run withdraw

  chmod +x ./run.sh
  ./run.sh
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
  $LEO run new
  $LEO run make_move

  chmod +x ./run.sh
  ./run.sh
)

# Build and run the simple token example programs.
(
  cd ./project/examples/simple_token || exit

  # Run the mint program.
  $LEO run mint

  # Run the transfer program.
  $LEO run transfer
)

# Build and run the token example program.
(
  cd ./project/examples/token || exit

  # Run the mint_public function.
  $LEO run mint_public

  # Run the mint_private function.
  $LEO run mint_private

  # Run the transfer_public function.
  $LEO run transfer_public

  # Run the transfer_private function.
  $LEO run transfer_private

  # Run the transfer_private_to_public function.
  $LEO run transfer_private_to_public

  # Run the transfer_public_to_private function.
  $LEO run transfer_public_to_private
)

# Build and run the two-adicity program.
(
  cd ./project/examples/twoadicity || exit
  $LEO run main
)

# Build and run the hackers-delight/ntzdebruijin program.
(
  cd ./project/examples/hackers-delight/ntzdebruijin || exit
  $LEO run
)

# Build and run the hackers-delight/ntzgaudet program.
(
  cd ./project/examples/hackers-delight/ntzgaudet || exit
  $LEO run
)

# Build and run the hackers-delight/ntzloops program.
(
  cd ./project/examples/hackers-delight/ntzloops || exit
  $LEO run
)

# Build and run the hackers-delight/ntzmasks program.
(
  cd ./project/examples/hackers-delight/ntzmasks || exit
  $LEO run
)

# Build and run the hackers-delight/ntzreisers program.
(
  cd ./project/examples/hackers-delight/ntzreisers || exit
  $LEO run
)

# Build and run the hackers-delight/ntzseals program.
(
  cd ./project/examples/hackers-delight/ntzseals || exit
  $LEO run
)

# Build and run the hackers-delight/ntzsearchtree program.
(
  cd ./project/examples/hackers-delight/ntzsearchtree || exit
  $LEO run
)

# Build and run the hackers-delight/ntzsmallvals program.
(
  cd ./project/examples/hackers-delight/ntzsmallvals || exit
  $LEO run
)
