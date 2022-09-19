# Build the Leo vote program.
(
  leo build || exit
)

# Run the `propose` program function
(
 leo run propose || exit
)
