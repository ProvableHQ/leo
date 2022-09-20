# Build the Leo vote program.
(
  leo build || exit
)

# Run the `propose` program function
(
 leo run propose || exit
)

# Run the `new_ticket` program function
(
  leo run new_ticket || exit
)

# Run the `agree` or `disagree` program function
(
  leo run agree || exit
  # leo run disagree || exit
)

