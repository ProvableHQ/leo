# Build and run the auction Leo program.
echo "Building and running the \`auction\` program..."
(
  cd ./project/examples/auction || exit
  $LEO run place_bid || exit
  $LEO run resolve || exit
  $LEO run finish || exit

  chmod +x ./run.sh || exit
  ./run.sh || exit
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
  cd ./project/examples/basic_bank || exit
  $LEO run issue || exit
  $LEO run deposit || exit
  $LEO run withdraw || exit

  chmod +x ./run.sh || exit
  ./run.sh || exit
)
# Check that the basic_bank program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`basic_bank\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the battleship Leo program.
echo "Building and running the \`battleship\` program..."
(
  cd ./project/examples/battleship || exit

  chmod +x ./run.sh || exit
  ./run.sh || exit
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
  cd ./project/examples/bubblesort || exit
  $LEO run bubblesort || exit
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
  cd ./project/examples/core || exit
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
  cd ./project/examples/groups || exit
  $LEO run main || exit
)
# Check that the groups program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`groups\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the hackers-delight/ntzdebruijin program.
echo "Building and running the \`hackers-delight/ntzdebruijin\` program..."
(
  cd ./project/examples/hackers-delight/ntzdebruijin || exit
  $LEO run || exit
)
# Check that the hackers-delight/ntzdebruijin program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`hackers-delight/ntzdebruijin\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the hackers-delight/ntzgaudet program.
echo "Building and running the \`hackers-delight/ntzgaudet\` program..."
(
  cd ./project/examples/hackers-delight/ntzgaudet || exit
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
  cd ./project/examples/hackers-delight/ntzloops || exit
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
  cd ./project/examples/hackers-delight/ntzmasks || exit
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
  cd ./project/examples/hackers-delight/ntzreisers || exit
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
  cd ./project/examples/hackers-delight/ntzseals || exit
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
  cd ./project/examples/hackers-delight/ntzsearchtree || exit
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
  cd ./project/examples/hackers-delight/ntzsmallvals || exit
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
  cd ./project/examples/helloworld || exit
  $LEO run main || exit
)
# Check that the helloworld program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`helloworld\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the import point example Leo program.
echo "Building and running the \`import_point\` program..."
(
  cd ./project/examples/import_point || exit
  $LEO run main || exit
)
# Check that the import point program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`import_point\` program failed to run successfully."
    exit $EXITCODE
fi

# Build and run the interest example Leo programs.
echo "Building and running the \`interest\` programs..."
(
  cd ./project/examples/import_point || exit

  # Run the fixed period interest program.
  $LEO run fixed_period_interest || exit

  # Run the bounded period interest program.
  $LEO run bounded_period_interest || exit
)
# Check that the interest programs ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`interest\` programs failed to run successfully."
    exit $EXITCODE
fi

# Build and run the message example Leo program.
echo "Building and running the \`message\` program..."
(
  cd ./project/examples/message || exit
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
  cd ./project/examples/tictactoe || exit
  $LEO run new || exit
  $LEO run make_move || exit

  chmod +x ./run.sh || exit
  ./run.sh || exit
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
  cd ./project/examples/simple_token || exit

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
  cd ./project/examples/token || exit

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
  cd ./project/examples/twoadicity || exit
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
  cd ./project/examples/vote || exit

  chmod +x ./run.sh || exit
  ./run.sh || exit
)
# Check that the vote program ran successfully.
EXITCODE=$?
if [ $EXITCODE -ne 0 ]; then
    echo "The \`vote\` program failed to run successfully."
    exit $EXITCODE
fi
