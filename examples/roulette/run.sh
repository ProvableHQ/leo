# Build the Leo roulette program.
(
  leo run build || exit
)

# Mint a new casino token.
(
  leo run mint_casino_token_record || exit
)

# Generate the seed for the roulette result.
(
  leo run psd_hash || exit
)

# Check the result of the last 6 bits mod(36)
(
  leo run psd_bits_mod || exit
)

# Execute the player bet
(
  leo run make_bet || exit
)