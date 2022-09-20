#!/bin/bash
# First check that Leo is installed.
if ! command -v leo &> /dev/null
then
    echo "leo is not installed."
    exit
fi

# Create a new game.
echo "


Creating a new game."
leo run new

# Have the first player make a move.
# | x |   |   |
# |   |   |   |
# |   |   |   |
echo "

The first player is making a move."
leo run make_move 1u8 1u8 1u8 "{ r1: { c1: 0u8, c2: 0u8, c3: 0u8 }, r2: { c1: 0u8, c2: 0u8, c3: 0u8 }, r3: { c1: 0u8, c2: 0u8, c3: 0u8 } }"

# Have the second player make a move.
# | x |   |   |
# |   | o |   |
# |   |   |   |
echo "

The second player is making a move."
leo run make_move 2u8 2u8 2u8 "{ r1: { c1: 1u8, c2: 0u8, c3: 0u8 }, r2: { c1: 0u8, c2: 0u8, c3: 0u8 }, r3: { c1: 0u8, c2: 0u8, c3: 0u8 } }"

# Have the first player make a move.
# | x |   |   |
# |   | o |   |
# | x |   |   |
echo "

The first player is making a move."
leo run make_move 1u8 3u8 1u8 "{ r1: { c1: 1u8, c2: 0u8, c3: 0u8 }, r2: { c1: 0u8, c2: 2u8, c3: 0u8 }, r3: { c1: 0u8, c2: 0u8, c3: 0u8 } }"

# Have the second player make a move.
# | x |   |   |
# | o | o |   |
# | x |   |   |
echo "

The second player is making a move."
leo run make_move 2u8 2u8 1u8 "{ r1: { c1: 1u8, c2: 0u8, c3: 0u8 }, r2: { c1: 0u8, c2: 2u8, c3: 0u8 }, r3: { c1: 1u8, c2: 0u8, c3: 0u8 } }"

# Have the first player make a move.
# | x |   |   |
# | o | o | x |
# | x |   |   |
echo "

The first player is making a move."
leo run make_move 1u8 2u8 3u8 "{ r1: { c1: 1u8, c2: 0u8, c3: 0u8 }, r2: { c1: 2u8, c2: 2u8, c3: 0u8 }, r3: { c1: 1u8, c2: 0u8, c3: 0u8 } }"

# Have the second player make a move.
# | x | o |   |
# | o | o | x |
# | x |   |   |
echo "

The second player is making a move."
leo run make_move 2u8 1u8 2u8 "{ r1: { c1: 1u8, c2: 0u8, c3: 0u8 }, r2: { c1: 2u8, c2: 2u8, c3: 1u8 }, r3: { c1: 1u8, c2: 0u8, c3: 0u8 } }"

# Have the first player make a move.
# | x | o |   |
# | o | o | x |
# | x | x |   |
echo "

The first player is making a move."
leo run make_move 1u8 3u8 2u8 "{ r1: { c1: 1u8, c2: 2u8, c3: 0u8 }, r2: { c1: 2u8, c2: 2u8, c3: 1u8 }, r3: { c1: 1u8, c2: 0u8, c3: 0u8 } }"

# Have the second player make a move.
# | x | o |   |
# | o | o | x |
# | x | x | o |
echo "

The second player is making a move."
leo run make_move 2u8 3u8 3u8 "{ r1: { c1: 1u8, c2: 2u8, c3: 0u8 }, r2: { c1: 2u8, c2: 2u8, c3: 1u8 }, r3: { c1: 1u8, c2: 1u8, c3: 0u8 } }"

# Have the first player make a move.
# | x | o | x |
# | o | o | x |
# | x | x | o |
echo "

The first player is making a move."
leo run make_move 1u8 1u8 3u8 "{ r1: { c1: 1u8, c2: 2u8, c3: 0u8 }, r2: { c1: 2u8, c2: 2u8, c3: 1u8 }, r3: { c1: 1u8, c2: 1u8, c3: 2u8 } }"
