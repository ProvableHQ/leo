# Alias the leo command to use the local binary.
# Note: Use a full path for $LEO when running locally.
leo() {
  $LEO "$@"
}

(
  # Create a new Leo lottery example program.
  $LEO example lottery || exit
  ls -la
  cd lottery && ls -la

  # Run the script.
  chmod +x ./run.sh || exit
  export -f leo
  ./run.sh || exit
)

(
  # Create a new Leo tictactoe example program.
  $LEO example tictactoe || exit
  ls -la
  cd tictactoe && ls -la

  # Run the script.
  chmod +x ./run.sh || exit
  export -f leo
  ./run.sh || exit
)

(
  #Create a new Leo token example program.
  $LEO example token || exit
  ls -la
  cd token && ls -la

  # Run the script.
  chmod +x ./run.sh || exit
  export -f leo
  ./run.sh || exit
)
