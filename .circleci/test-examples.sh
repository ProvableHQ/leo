# Alias the leo command to use the local binary.
# Note: Use a full path for $LEO when running locally.
leo() {
  $LEO "$@"
}

# Build and run the auction Leo program.
echo "Building and running the \`auction\` program..."
(
  cd $EXAMPLES/auction || exit
  $LEO run place_bid || exit
  $LEO run resolve || exit
  $LEO run finish || exit

  chmod +x $EXAMPLES/auction/run.sh || exit
  export -f leo || exit
  $EXAMPLES/auction/run.sh || exit
)
# Check that the auction program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`auction\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the basic_bank Leo program.
echo "Building and running the \`basic_bank\` program..."
(
  cd $EXAMPLES/basic_bank || exit
  $LEO run issue || exit
  $LEO run deposit || exit
  $LEO run withdraw || exit

  chmod +x $EXAMPLES/basic_bank/run.sh || exit
  export -f leo || exit
  $EXAMPLES/basic_bank/run.sh || exit
)
# Check that the basic_bank program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`basic_bank\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the battleship Leo program.
echo "Building and running the \`battleship\` program..."
which leo
(
  cd $EXAMPLES/battleship || exit

  chmod +x $EXAMPLES/battleship/run.sh || exit
  export -f leo || exit
  $EXAMPLES/battleship/run.sh || exit
)
# Check that the battleship program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`battleship\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the bubblesort Leo program.
echo "Building and running the \`bubblesort\` program..."
(
  cd $EXAMPLES/bubblesort || exit
  $LEO run bubble_sort || exit
)
# Check that the bubblesort program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`bubblesort\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the core example Leo program.
echo "Building and running the \`core\` program..."
(
  cd $EXAMPLES/core || exit
  $LEO run main || exit
)
# Check that the core program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`core\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the groups example Leo program.
echo "Building and running the \`groups\` program..."
(
  cd $EXAMPLES/groups || exit
  $LEO run main || exit
)
# Check that the groups program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`groups\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the hackers-delight/ntzdebruijn program.
echo "Building and running the \`hackers-delight/ntzdebruijn\` program..."
(
  cd $EXAMPLES/hackers-delight/ntzdebruijn || exit
  $LEO run || exit
)
# Check that the hackers-delight/ntzdebruijn program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`hackers-delight/ntzdebruijn\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the hackers-delight/ntzgaudet program.
echo "Building and running the \`hackers-delight/ntzgaudet\` program..."
(
  cd $EXAMPLES/hackers-delight/ntzgaudet || exit
  $LEO run || exit
)
# Check that the hackers-delight/ntzgaudet program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`hackers-delight/ntzgaudet\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the hackers-delight/ntzloops program.
echo "Building and running the \`hackers-delight/ntzloops\` program..."
(
  cd $EXAMPLES/hackers-delight/ntzloops || exit
  $LEO run || exit
)
# Check that the hackers-delight/ntzloops program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`hackers-delight/ntzloops\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the hackers-delight/ntzmasks program.
echo "Building and running the \`hackers-delight/ntzmasks\` program..."
(
  cd $EXAMPLES/hackers-delight/ntzmasks || exit
  $LEO run || exit
)
# Check that the hackers-delight/ntzmasks program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`hackers-delight/ntzmasks\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the hackers-delight/ntzreisers program.
echo "Building and running the \`hackers-delight/ntzreisers\` program..."
(
  cd $EXAMPLES/hackers-delight/ntzreisers || exit
  $LEO run || exit
)
# Check that the hackers-delight/ntzreisers program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`hackers-delight/ntzreisers\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the hackers-delight/ntzseals program.
echo "Building and running the \`hackers-delight/ntzseals\` program..."
(
  cd $EXAMPLES/hackers-delight/ntzseals || exit
  $LEO run || exit
)
# Check that the hackers-delight/ntzseals program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`hackers-delight/ntzseals\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the hackers-delight/ntzsearchtree program.
echo "Building and running the \`hackers-delight/ntzsearchtree\` program..."
(
  cd $EXAMPLES/hackers-delight/ntzsearchtree || exit
  $LEO run || exit
)
# Check that the hackers-delight/ntzsearchtree program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`hackers-delight/ntzsearchtree\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the hackers-delight/ntzsmallvals program.
echo "Building and running the \`hackers-delight/ntzsmallvals\` program..."
(
  cd $EXAMPLES/hackers-delight/ntzsmallvals || exit
  $LEO run || exit
)
# Check that the hackers-delight/ntzsmallvals program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`hackers-delight/ntzsmallvals\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the helloworld Leo program.
echo "Building and running the \`helloworld\` program..."
(
  cd $EXAMPLES/helloworld || exit
  $LEO run main || exit
)
# Check that the helloworld program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`helloworld\` program failed to run successfully."
    exit $EXITCODE
fi


# Build and run the interest example Leo programs.
echo "Building and running the \`interest\` programs..."
(
  cd $EXAMPLES/interest || exit

  # Run the fixed period interest program.
  $LEO run fixed_iteration_interest || exit

  # Run the bounded period interest program.
  $LEO run bounded_iteration_interest || exit
)
# Check that the interest programs ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`interest\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the message example Leo program.
echo "Building and running the \`message\` program..."
(
  cd $EXAMPLES/message || exit
  $LEO run main || exit
)
# Check that the message program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`message\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the tic tac toe example Leo program.
echo "Building and running the \`tictactoe\` program..."
(
  cd $EXAMPLES/tictactoe || exit
  $LEO run new || exit
  $LEO run make_move || exit

  chmod +x $EXAMPLES/tictactoe/run.sh || exit
  export -f leo
  $EXAMPLES/tictactoe/run.sh || exit
)
# Check that the tic tac toe program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`tictactoe\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the simple token example programs.
echo "Building and running the \`simple_token\` programs..."
(
  cd $EXAMPLES/simple_token || exit

  # Run the mint program.
  $LEO run mint

  # Run the transfer program.
  $LEO run transfer
)
# Check that the simple token programs ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`simple_token\` programs failed to run successfully."
    exit $EXITCODE
fi

# Build and run the token example program.
echo "Building and running the \`token\` program..."
(
  cd $EXAMPLES/token || exit

  # Run the mint_public function.
  $LEO run mint_public || exit

  # Run the mint_private function.
  $LEO run mint_private || exit

  # Run the transfer_public function.
  $LEO run transfer_public || exit

  # Run the transfer_private function.
  $LEO run transfer_private || exit

  # Run the transfer_private_to_public function.
  $LEO run transfer_private_to_public || exit

  # Run the transfer_public_to_private function.
  $LEO run transfer_public_to_private || exit
)
# Check that the token program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`token\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the two-adicity program.
echo "Building and running the \`twoadicity\` program..."
(
  cd $EXAMPLES/twoadicity || exit
  $LEO run main || exit
)
# Check that the two-adicity program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`twoadicity\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the vote Leo program.
echo "Building and running the \`vote\` program..."
(
  cd $EXAMPLES/vote || exit

  chmod +x $EXAMPLES/vote/run.sh || exit
  export -f leo
  $EXAMPLES/vote/run.sh || exit
)
# Check that the vote program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`vote\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the lottery Leo program.
echo "Building and running the \`lottery\` program..."
(
  cd $EXAMPLES/lottery || exit

  chmod +x $EXAMPLES/lottery/run.sh || exit
  export -f leo
  $EXAMPLES/lottery/run.sh || exit
)
# Check that the lottery program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`lottery\` program failed to run successfully."
    exit $EXITCODE
fi