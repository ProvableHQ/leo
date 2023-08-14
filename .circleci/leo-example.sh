(
  # Create a new Leo lottery example program.
  $LEO example lottery || exit
  ls -la
  cd lottery && ls -la

  # Run the play function.
  $LEO run play || exit
)

(
  # Create a new Leo tictactoe example program.
  $LEO example tictactoe || exit
  ls -la
  cd tictactoe && ls -la

  # Create a new game.
  $LEO run new || exit

  # Create a make a move.
  $LEO run make_move || exit
)

(
  #Create a new Leo token example program.
  $LEO example token || exit
  ls -la
  cd token && ls -la

  # Run the mint_public function.
  $LEO run mint_public || exit
)
