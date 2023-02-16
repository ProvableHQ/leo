# Alias the Leo binary.
alias leo="$LEO"

# Build and run the auction Leo program.
echo "Building and running the \`auction\` program..."
(
  cd ./project/examples/auction || exit
  $LEO run place_bid
  $LEO run resolve
  $LEO run finish

  chmod +x ./run.sh
  ./run.sh
)

# Build and run the basic_bank Leo program.
echo "Building and running the \`basic_bank\` program..."
(
  cd ./project/examples/basic_bank || exit
  $LEO run issue
  $LEO run deposit
  $LEO run withdraw

  chmod +x ./run.sh
  ./run.sh
)

# Build and run the battleship Leo program.
echo "Building and running the \`battleship\` program..."
(
  cd ./project/examples/battleship || exit

  chmod +x ./run.sh
  ./run.sh
)

# Build and run the bubblesort Leo program.
echo "Building and running the \`bubblesort\` program..."
(
  cd ./project/examples/bubblesort || exit
  $LEO run bubblesort
)

# Build and run the core example Leo program.
echo "Building and running the \`core\` program..."
(
  cd ./project/examples/core || exit
  $LEO run main
)

# Build and run the groups example Leo program.
echo "Building and running the \`groups\` program..."
(
  cd ./project/examples/groups || exit
  $LEO run main
)

# Build and run the hackers-delight/ntzdebruijin program.
echo "Building and running the \`hackers-delight/ntzdebruijin\` program..."
(
  cd ./project/examples/hackers-delight/ntzdebruijin || exit
  $LEO run
)

# Build and run the hackers-delight/ntzgaudet program.
echo "Building and running the \`hackers-delight/ntzgaudet\` program..."
(
  cd ./project/examples/hackers-delight/ntzgaudet || exit
  $LEO run
)

# Build and run the hackers-delight/ntzloops program.
echo "Building and running the \`hackers-delight/ntzloops\` program..."
(
  cd ./project/examples/hackers-delight/ntzloops || exit
  $LEO run
)

# Build and run the hackers-delight/ntzmasks program.
echo "Building and running the \`hackers-delight/ntzmasks\` program..."
(
  cd ./project/examples/hackers-delight/ntzmasks || exit
  $LEO run
)

# Build and run the hackers-delight/ntzreisers program.
echo "Building and running the \`hackers-delight/ntzreisers\` program..."
(
  cd ./project/examples/hackers-delight/ntzreisers || exit
  $LEO run
)

# Build and run the hackers-delight/ntzseals program.
echo "Building and running the \`hackers-delight/ntzseals\` program..."
(
  cd ./project/examples/hackers-delight/ntzseals || exit
  $LEO run
)

# Build and run the hackers-delight/ntzsearchtree program.
echo "Building and running the \`hackers-delight/ntzsearchtree\` program..."
(
  cd ./project/examples/hackers-delight/ntzsearchtree || exit
  $LEO run
)

# Build and run the hackers-delight/ntzsmallvals program.
echo "Building and running the \`hackers-delight/ntzsmallvals\` program..."
(
  cd ./project/examples/hackers-delight/ntzsmallvals || exit
  $LEO run
)

# Build and run the helloworld Leo program.
echo "Building and running the \`helloworld\` program..."
(
  cd ./project/examples/helloworld || exit
  $LEO run main
)

# Build and run the import point example Leo program.
echo "Building and running the \`import_point\` program..."
(
  cd ./project/examples/import_point || exit
  $LEO run main
)

# Build and run the interest example Leo programs.
echo "Building and running the \`interest\` programs..."
(
  cd ./project/examples/import_point || exit

  # Run the fixed period interest program.
  $LEO run fixed_period_interest

  # Run the bounded period interest program.
  $LEO run bounded_period_interest
)

# Build and run the message example Leo program.
echo "Building and running the \`message\` program..."
(
  cd ./project/examples/message || exit
  $LEO run main
)

# Build and run the tic tac toe example Leo program.
echo "Building and running the \`tictactoe\` program..."
(
  cd ./project/examples/tictactoe || exit
  $LEO run new
  $LEO run make_move

  chmod +x ./run.sh
  ./run.sh
)

# Build and run the simple token example programs.
echo "Building and running the \`simple_token\` programs..."
(
  cd ./project/examples/simple_token || exit

  # Run the mint program.
  $LEO run mint

  # Run the transfer program.
  $LEO run transfer
)

# Build and run the token example program.
echo "Building and running the \`token\` program..."
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
echo "Building and running the \`twoadicity\` program..."
(
  cd ./project/examples/twoadicity || exit
  $LEO run main
)

# Build and run the vote Leo program.
echo "Building and running the \`vote\` program..."
(
  cd ./project/examples/vote || exit

  chmod +x ./run.sh
  ./run.sh
)
