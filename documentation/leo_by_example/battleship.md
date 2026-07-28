---
id: battleship
title: A Game of Battleship in Leo
---

[general tags]: # "example, battleship, struct, program"

**[Source Code](https://github.com/ProvableHQ/leo-examples/tree/main/battleship)**

## Contents

- [Contents](#contents)
- [Summary](#summary)
- [How to Run](#how-to-run)
- [1. Initializing the Players](#1-initializing-the-players)
- [2. Player 1 Places Ships on the Board](#2-player-1-places-ships-on-the-board)
- [3: Player 1 Passes The Board To Player 2](#3-player-1-passes-the-board-to-player-2)
- [4: Player 2 Places Ships On The Board](#4-player-2-places-ships-on-the-board)
- [5: Passing The Board Back To Player 1](#5-passing-the-board-back-to-player-1)
- [6: Player 1 Takes The 1st Turn](#6-player-1-takes-the-1st-turn)
- [7: Player 2 Takes The 2nd Turn](#7-player-2-takes-the-2nd-turn)
- [8: Player 1 Takes The 3rd Turn](#8-player-1-takes-the-3rd-turn)
- [9: Player 2 Takes The 4th Turn](#9-player-2-takes-the-4th-turn)
- [10. Who Wins?](#10-who-wins)
- [ZK Battleship Privacy](#zk-battleship-privacy)
- [Modeling the board and ships](#modeling-the-board-and-ships)
  - [Examples of valid board configurations:](#examples-of-valid-board-configurations)
  - [Examples of invalid board configurations:](#examples-of-invalid-board-configurations)
- [Validating a single ship at a time](#validating-a-single-ship-at-a-time)
  - [Bit Counting](#bit-counting)
  - [Adjacency Check](#adjacency-check)
  - [Splitting a row or column](#splitting-a-row-or-column)
  - [Ensuring a bitstring is a power of 2](#ensuring-a-bitstring-is-a-power-of-2)
- [Validating all ships together in a single board](#validating-all-ships-together-in-a-single-board)
- [Ensure that players and boards cannot swap mid-game](#ensure-that-players-and-boards-cannot-swap-mid-game)
- [Ensure that each player can only move once before the next player can move](#ensure-that-each-player-can-only-move-once-before-the-next-player-can-move)
- [Enforce valid moves](#enforce-valid-moves)
- [Winning the game](#winning-the-game)

## Summary

This Battleship implementation showcases a well-designed application within Leo’s current constraints. However, some aspects—especially the bit manipulation—might seem complex at first glance. To set expectations, this is a more advanced example due to the way the board is encoded and manipulated. Planned improvements to Leo could make implementations like this much simpler in the future.

In Battleship, two players put ships in secret positions on separate 8x8 grids.
The players then take turns to fire at the other player's board.
The game ends when one player has sunk all of the other player's ships.

This application is a Leo translation of the Aleo community's
[zk-battleship](https://github.com/demox-labs/zk-battleship) example.

## How to Run

Follow the [Leo Installation Instructions](https://docs.leo-lang.org/getting_started/installation).

This battleship program can be run using the following bash script. Locally, it will execute Leo program functions to create the board, place ships, and play a game of battleship.

```bash
cd battleship
./run.sh
```

The `.env` file contains a private key and address. This is the account that will be used to sign transactions and is checked for record ownership. When executing programs as different parties, be sure to set the `private_key` field in `.env` to the appropriate value. You can check out how we have set things up in `./run.sh` for a full example of how to run the program as different parties.

## 1. Initializing the Players

In order to play battleship, there must be two players with two boards. Players will be represented by their Aleo address.

We will be playing the role of these two parties:

```bash
The private key and address of player 1.
private_key: APrivateKey1zkpGKaJY47BXb6knSqmT3JZnBUEGBDFAWz2nMVSsjwYpJmm
address: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy

The private key and address of player 2.
private_key: APrivateKey1zkp86FNGdKxjgAdgQZ967bqBanjuHkAaoRe19RK24ZCGsHH
address: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry
```

## 2. Player 1 Places Ships on the Board

Now, we need to make a board as Player 1. See the [modeling the boards and ships](#modeling-the-board-and-ships) section for information on valid ship bitstrings and placements on the board.

With player 1's private key, they initialize the board with the placement of 4 ships and the opponent's public address.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkpGKaJY47BXb6knSqmT3JZnBUEGBDFAWz2nMVSsjwYpJmm
" > .env

leo run initialize_board 34084860461056u64 551911718912u64 7u64 1157425104234217472u64 aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry
```

```bash
➡️  Output

 • {
  owner: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  hits_and_misses: 0u64.private,
  played_tiles: 0u64.private,
  ships: 1157459741006397447u64.private,
  player_1: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  player_2: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  game_started: false.private,
  _nonce: 605849623036268790365773177565562473735086364071033205649960161942593750353group.public
}

Leo ✅ Finished 'battleship.aleo::initialize_board'
```

The output is a board_state record owned by Player 1.
Notice that the `game_started` flag is false, as well as the composite ship configuration `ships`. 1157459741006397447u64 to a binary bitstring becomes `0001000000010000000111111000000010000000100000001000000000000111`,
or laid out in columns and rows:

```text
0 0 0 1 0 0 0 0
0 0 0 1 0 0 0 0
0 0 0 1 1 1 1 1
1 0 0 0 0 0 0 0
1 0 0 0 0 0 0 0
1 0 0 0 0 0 0 0
1 0 0 0 0 0 0 0
0 0 0 0 0 1 1 1
```

## 3: Player 1 Passes The Board To Player 2

Now, we can offer a battleship game to player 2. Run `offer_battleship` with the record you just created:

```bash
leo run offer_battleship "{
  owner: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  hits_and_misses: 0u64.private,
  played_tiles: 0u64.private,
  ships: 1157459741006397447u64.private,
  player_1: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  player_2: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  game_started: false.private,
  _nonce: 605849623036268790365773177565562473735086364071033205649960161942593750353group.public
}"
```

```bash
➡️  Outputs

 • {
  owner: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  hits_and_misses: 0u64.private,
  played_tiles: 0u64.private,
  ships: 1157459741006397447u64.private,
  player_1: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  player_2: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  game_started: true.private,
  _nonce: 5443521912126792569907060514335205174032013684291524549930033539632156136027group.public
}
 • {
  owner: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  incoming_fire_coordinate: 0u64.private,
  player_1: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  player_2: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  prev_hit_or_miss: 0u64.private,
  _nonce: 6986401140057557061321899375524513841643724821230599181456639629979203966487group.public
}

Leo ✅ Finished 'battleship.aleo::offer_battleship'
```

The first output record is the updated `board_state.record`. The `game_started` flag is now `true`.
This board cannot offer or accept another Battleship game. Player 1 must initialize a new board for another game.

The second output record is a dummy `move.record`. It does not contain fire coordinates or information about Player 2's moves.
Player 2 owns this record. To accept the game, Player 2 must use it with their `board_state.record`.

## 4: Player 2 Places Ships On The Board

We switch our .env to player 2's private key and similarly run initialize_board to create a new and different board for player two.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkp86FNGdKxjgAdgQZ967bqBanjuHkAaoRe19RK24ZCGsHH
" > .env

leo run initialize_board 31u64 2207646875648u64 224u64 9042383626829824u64 aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy
```

```bash
➡️  Output

 • {
  owner: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  hits_and_misses: 0u64.private,
  played_tiles: 0u64.private,
  ships: 9044591273705727u64.private,
  player_1: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  player_2: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  game_started: false.private,
  _nonce: 677929557867990662961068737825412945684193990901139603462104629310061710321group.public
}

✅ Executed 'battleship.aleo::initialize_board'
```

Note, the output ships here is 9044591273705727u64, which in a bitstring is:

```text
0 0 1 0 0 0 0 0
0 0 1 0 0 0 1 0
0 0 0 0 0 0 1 0
0 0 0 0 0 0 1 0
0 0 0 0 0 0 1 0
0 0 0 0 0 0 0 0
1 1 1 1 1 1 1 1
```

## 5: Passing The Board Back To Player 1

Now, we can accept Player 1's offer. Run `start_battleship`:

```bash
leo run start_battleship "{
  owner: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  hits_and_misses: 0u64.private,
  played_tiles: 0u64.private,
  ships: 9044591273705727u64.private,
  player_1: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  player_2: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  game_started: false.private,
  _nonce: 677929557867990662961068737825412945684193990901139603462104629310061710321group.public
}" "{
  owner: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  incoming_fire_coordinate: 0u64.private,
  player_1: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  player_2: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  prev_hit_or_miss: 0u64.private,
  _nonce: 6306786918362462465996698473371289503655844751914031374264794338640697795225group.public
}"
```

```bash
➡️  Outputs

 • {
  owner: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  hits_and_misses: 0u64.private,
  played_tiles: 0u64.private,
  ships: 9044591273705727u64.private,
  player_1: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  player_2: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  game_started: true.private,
  _nonce: 499506036017893504519951074816367233238764881167148207158107765834843789278group.public
}
 • {
  owner: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  incoming_fire_coordinate: 0u64.private,
  player_1: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  player_2: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  prev_hit_or_miss: 0u64.private,
  _nonce: 7551593771072417773015833444631669906818701068612998340960968556531564726874group.public
}

✅ Executed 'battleship.aleo::start_battleship'
```

Notice the outputs here are similar to `offer_battleship`. A dummy `move.record` is owned by Player 1, and Player 2 gets a `board_state.record` with the `game_started` flag updated. However, now that Player 1 has a `move.record` and a started board, they can begin to play.

## 6: Player 1 Takes The 1st Turn

We switch the .env back to player 1, and we run the `play` function.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkpGKaJY47BXb6knSqmT3JZnBUEGBDFAWz2nMVSsjwYpJmm
" > .env

leo run play "{
  owner: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  hits_and_misses: 0u64.private,
  played_tiles: 0u64.private,
  ships: 1157459741006397447u64.private,
  player_1: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  player_2: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  game_started: true.private,
  _nonce: 6313341191294792052861773157032837489809107102476040695601777954897783350080group.public
}" "{
  owner: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  incoming_fire_coordinate: 0u64.private,
  player_1: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  player_2: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  prev_hit_or_miss: 0u64.private,
  _nonce: 2798663115519921626400765401803177719929914180089719334947022448579691220488group.public
}" 1u64
```

```bash
➡️  Outputs

 • {
  owner: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  hits_and_misses: 0u64.private,
  played_tiles: 1u64.private,
  ships: 1157459741006397447u64.private,
  player_1: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  player_2: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  game_started: true.private,
  _nonce: 5833516448655036599597838063894464861371198938108460526636526325286738488235group.public
}
 • {
  owner: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  incoming_fire_coordinate: 1u64.private,
  player_1: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  player_2: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  prev_hit_or_miss: 0u64.private,
  _nonce: 4383078917685812690935470339923943658033179718952229417171392956492546325808group.public
}

✅ Executed 'battleship.aleo::play'
```

Player 1 has an updated `board_state.record`. Its new `played_tiles` bitstring contains the fire coordinate sent to Player 2.
The `incoming_fire_coordinate` in Player 2's `move.record` matches Player 1's input.
Player 2 can use this move tile and send a new fire coordinate.
The response also tells Player 1 if their coordinate hit one of Player 2's ships.

## 7: Player 2 Takes The 2nd Turn

We switch the .env back to player 2, and we run the `play` function.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkp86FNGdKxjgAdgQZ967bqBanjuHkAaoRe19RK24ZCGsHH
" > .env

leo run play "{
  owner: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  hits_and_misses: 0u64.private,
  played_tiles: 0u64.private,
  ships: 9044591273705727u64.private,
  player_1: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  player_2: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  game_started: true.private,
  _nonce: 6864275139988909612799168784231775829713739147830284979332684562641318182923group.public
}" "{
  owner: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  incoming_fire_coordinate: 1u64.private,
  player_1: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  player_2: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  prev_hit_or_miss: 0u64.private,
  _nonce: 8420474443174402614458578667801578345975509805478103542095622903412594983971group.public
}" 2048u64
```

```bash
➡️  Outputs

 • {
  owner: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  hits_and_misses: 0u64.private,
  played_tiles: 2048u64.private,
  ships: 9044591273705727u64.private,
  player_1: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  player_2: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  game_started: true.private,
  _nonce: 6284479302801058138006361960649628992876976428745392660731784830148359328839group.public
}
 • {
  owner: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  incoming_fire_coordinate: 2048u64.private,
  player_1: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  player_2: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  prev_hit_or_miss: 1u64.private,
  _nonce: 8217837260140600949756911248177622179381338760298068527463640818659709985441group.public
}

✅ Executed 'battleship.aleo::play'
```

Player 2 now has an updated `board_state.record` which includes their newly updated `played_tiles`, only containing the fire coordinate they just sent to Player 1. Player 1 now owns a new `move.record` which includes the `hits_and_misses` field.
This field contains only the result of Player 1's previous fire coordinate. A hit contains one coordinate on the 8x8 grid.
A miss is `0u64`, which represents an 8x8 grid of zeros. A hit is the `u64` bitstring for the previous coordinate.

Two of Player 2's ships cover the complete bottom row. Thus, the following values are valid hits on that row:

- `1u64`
- `2u64`
- `4u64`
- `8u64`
- `16u64`
- `32u64`
- `64u64`
- `128u64`

Player 1's first fire coordinate was `1u64`, and it was a hit. Therefore, the `hits_and_misses` field is also `1u64`.

Player 1's next move consumes this `move.record` and updates Player 1's board with the hit or miss.
The move also calculates the result of Player 2's fire coordinate.
Player 1 now has values in `played_tiles` and cannot select a previous coordinate.
For example, `aleo run play 'board_state.record' 'move.record' 1u64` fails because Player 1 already used `1u64`.

## 8: Player 1 Takes The 3rd Turn

We switch the .env back to player 1, and we run the `play` function.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkpGKaJY47BXb6knSqmT3JZnBUEGBDFAWz2nMVSsjwYpJmm
" > .env

leo run play "{
  owner: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  hits_and_misses: 0u64.private,
  played_tiles: 1u64.private,
  ships: 1157459741006397447u64.private,
  player_1: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  player_2: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  game_started: true.private,
  _nonce: 1962122153746742645258971561783872712461616481157617568489391338473028502271group.public
}" "{
  owner: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  incoming_fire_coordinate: 2048u64.private,
  player_1: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  player_2: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  prev_hit_or_miss: 1u64.private,
  _nonce: 1204008848449868423802652577996848559012797694551224583683080100053831915439group.public
}" 2u64
```

```bash
➡️  Outputs

 • {
  owner: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  hits_and_misses: 1u64.private,
  played_tiles: 3u64.private,
  ships: 1157459741006397447u64.private,
  player_1: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  player_2: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  game_started: true.private,
  _nonce: 5338125050531864311985370830280952305688629865354830939402745656578990650505group.public
}
 • {
  owner: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  incoming_fire_coordinate: 2u64.private,
  player_1: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  player_2: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  prev_hit_or_miss: 0u64.private,
  _nonce: 7971995563631235472540847437984726419106193784727086463494463811056252801811group.public
}

✅ Executed 'battleship.aleo::play'
```

As before, both a `board_state.record` and `move.record` are created. The `board_state.record` now contains 3u64 as the `played_tiles`, which looks like this in bitstring form:

```text
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 1 1
```

The `hits_and_misses` field in `board_state.record` contains the result of the previous move.
Player 2's new `move.record` contains the result of Player 2's previous move. It also contains Player 1's new fire coordinate.

## 9: Player 2 Takes The 4th Turn

We switch the .env back to player 2, and we run the `play` function.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkp86FNGdKxjgAdgQZ967bqBanjuHkAaoRe19RK24ZCGsHH
" > .env

leo run play "{
  owner: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  hits_and_misses: 0u64.private,
  played_tiles: 2048u64.private,
  ships: 9044591273705727u64.private,
  player_1: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  player_2: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  game_started: true.private,
  _nonce: 591128247205636061702123861968396246163831838278146623498909560875485861872group.public
}" "{
  owner: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  incoming_fire_coordinate: 2u64.private,
  player_1: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  player_2: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  prev_hit_or_miss: 0u64.private,
  _nonce: 4871574741887919250014604645502780786361650856453535231083359604148337116539group.public
}" 4u64
```

```bash
➡️  Outputs

 • {
  owner: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  hits_and_misses: 0u64.private,
  played_tiles: 2052u64.private,
  ships: 9044591273705727u64.private,
  player_1: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  player_2: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  game_started: true.private,
  _nonce: 4866144015676673398767235148516158177034901439767024502676546368462039477864group.public
}
 • {
  owner: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  incoming_fire_coordinate: 4u64.private,
  player_1: aleo1wyvu96dvv0auq9e4qme54kjuhzglyfcf576h0g3nrrmrmr0505pqd6wnry.private,
  player_2: aleo15g9c69urtdhvfml0vjl8px07txmxsy454urhgzk57szmcuttpqgq5cvcdy.private,
  prev_hit_or_miss: 2u64.private,
  _nonce: 5304512645876453228434639693756897952439730718508628026257897445388710294282group.public
}

✅ Executed 'battleship.aleo::play'
```

## 10. Who Wins

Play continues back and forth between Player 1 and Player 2. When one player has a total of 14 flipped bits in their `hits_and_misses` field on their `board_state.record`, they have won the game.

## ZK Battleship Privacy

How can we ensure that the ship configurations of each player remains secret,
while being able to trustlessly and fairly play with their opponent?
By taking advantage of selective privacy powered by zero knowledge proofs on Aleo.

Broadly speaking, we can follow this general strategy:

1. Create mathematical rules for ship positions.
   These rules prevent players from stacking, removing, or intersecting their ships.

2. Ensure that the players and boards that begin a game cannot be swapped out.

3. Ensure that each player can only move once before the next player can move.

4. Enforce constraints on valid moves, and force the player to give their opponent information about their opponent's previous move in order to continue playing.

## Modeling the board and ships

Most Battleship programs use a 64-character string or eight arrays with eight elements to represent the board.
Leo does not yet have efficient string support or `for` and `while` loops. However, Aleo has the unsigned 64-bit integer type `u64`.
Each bit in a `u64` can represent one position on a Battleship board. For example, the following value represents an empty board:
0u64 =

```text
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
```

This Battleship game has four ships with lengths 5, 4, 3, and 2. Other versions can have more ships.
This project uses the basic four-ship version.
A valid ship must be horizontal or vertical. A ship cannot cross a row boundary or intersect another ship.
Ships can touch each other.

Similar to how we represent a board with a u64 bitstring, we can represent a ship horizontally as a bitstring. We "flip" the bits to represent a ship:

| Length | Bitstring | u64   |
| ------ | --------- | ----- |
| 5      | 11111     | 31u64 |
| 4      | 1111      | 15u64 |
| 3      | 111       | 7u64  |
| 2      | 11        | 3u64  |

We can also represent a ship vertically as a bitstring. To show this, we need 7 "unflipped" bits (zeroes) in between the flipped bits so that the bits are adjacent vertically.

| Length | Bitstring                             | u64           |
| ------ | ------------------------------------- | ------------- |
| 5      | 1 00000001 00000001 00000001 00000001 | 4311810305u64 |
| 4      | 1 00000001 00000001 00000001          | 16843009u64   |
| 3      | 1 00000001 00000001                   | 65793u64      |
| 2      | 1 00000001                            | 257u64        |

With a board model and ship bitstring models, we can now place ships on a board.

### Examples of valid board configurations

17870284429256033024u64

```text
1 1 1 1 1 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 1
0 0 0 0 0 0 0 1
1 1 1 1 0 0 0 1
0 0 0 0 0 0 0 0
0 0 0 0 0 0 1 1
0 0 0 0 0 0 0 0
```

16383u64

```text
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 1 1 1 1 1 1
1 1 1 1 1 1 1 1
```

2157505700798988545u64

```text
0 0 0 1 1 1 0 1
1 1 1 1 0 0 0 1
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 1
0 0 0 0 0 0 0 1
0 0 0 0 0 0 0 1
0 0 0 0 0 0 0 1
0 0 0 0 0 0 0 1
```

### Examples of invalid board configurations

Ships overlapping the bottom ship:  
67503903u64

```text
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 1 0 0
0 0 0 0 0 1 1 0
0 0 0 0 0 1 1 1
0 0 0 1 1 1 1 1
```

Diagonal ships:  
9242549787790754436u64

```text
1 0 0 0 0 0 0 0
0 1 0 0 0 1 0 0
0 0 1 0 0 0 1 0
0 0 0 1 0 0 0 0
0 0 0 1 1 0 0 0
0 0 1 0 0 0 0 1
0 1 0 0 0 0 1 0
1 0 0 0 0 1 0 0
```

Ships splitting across rows and columns:  
1297811850814034450u64

```text
0 0 0 1 0 0 1 0
0 0 0 0 0 0 1 0
1 1 0 0 0 0 0 1
0 0 0 0 0 0 0 0
1 0 0 1 0 0 0 1
0 0 0 1 0 0 0 0
0 0 0 1 0 0 1 0
0 0 0 1 0 0 1 0
```

First, validate the bitstring position of each ship. If all positions are valid, combine them on one board.
Then, validate the complete board. The complete board is valid if the valid ship positions do not overlap.

## Validating a single ship at a time

To follow along with the code, all verification of ship bitstrings is done in verify.aleo. We know a ship is valid if all these conditions are met:
If horizontal:

1. The correct number of bits is flipped (a ship of length 5 should not have 6 flipped bits)
2. All the bits are adjacent to each other.
3. The bits do not split a row.

If vertical:

1. The correct number of bits is flipped.
2. All the bits are adjacent to each other, vertically. This means that each flipped bit should be separated by exactly 7 unflipped bits.
3. The bits do not split a column.

If a ship is valid vertically or horizontally, then we know the ship is valid. We just need to check for the bit count, the adjacency of those bits, and make sure those bits do not split a row/column. However, we cannot loop through the bit string to count bits, or to make sure those bits do not break across columns. We will need to turn to special bitwise operations and hacks.

### Bit Counting

See the `c_bitcount` closure in the code. The MIT AI Laboratory published
[HAKMEM](https://www.jjj.de/hakmem/hakmem.html), a collection of methods for fast bitwise operations.
HAKMEM 169 is the basis for this bit-count method. The implementation uses a modified form that is easier to understand.

Let a,b,c,d be either 0 or 1. Given a polynomial 8a + 4b + 2c + d, how do we find the summation of a + b + c + d? If we subtract subsets of this polynomial, we will be left with the summation.

Step 1: 8a + 4b + 2c + d  
Step 2: -4a - 2b - c  
Step 3: -2a - b  
Step 4: - a  
Step 5: = a + b + c + d

This polynomial is a bitwise representation of a number. Use these instructions to get the bit count of `1011`, or `13u64`.
Step 2 subtracts the start value after one right shift. This operation is equivalent to division by 2.
Step 3 subtracts the start value after two right shifts. Step 4 subtracts the value after three right shifts.

Thus, for a four-digit binary number `A`, use `A - (A >> 1) - (A >> 2) - (A >> 3) = B`.

Step 1: 1101 = 13u64  
Step 2: -0110 = 6u64  
Step 3: -0011 = 3u64  
Step 4: -0001 = 1u64  
Step 5: =0011 = 3u64

Use bit masks to apply this process to a number of any bit length.
The masks separate the sums into four-bit groups and prevent interference between adjacent groups.
For a larger start value such as `1111 0001 0111 0110`, use the following bit masks:

```text
For A >> 1, we'll use 0111 0111 0111 .... (in u64, this is 8608480567731124087u64)
For A >> 2, we'll use 0011 0011 0011 .... (in u64, this is 3689348814741910323u64)
For A >> 3, we'll use 0001 0001 0001 .... (in u64, this is 1229782938247303441u64)
```

For example, finding the sums of groups of 4 with a 16-bit number we will call A to yield the bit sum number B:

```text
A:    1111 0001 0111 0110
A>>1: 0111 1000 1011 1011
A>>2: 0011 1100 0101 1101
A>>3: 0001 1110 0010 1110

A>>1: 0111 1000 1011 1011
    & 0111 0111 0111 0111:
      0111 0000 0011 0011

A>>2: 0011 1100 0101 1101
    & 0011 0011 0011 0011:
      0011 0000 0001 0001

A>>3: 0001 1110 0010 1110
    & 0001 0001 0001 0001:
      0001 0000 0000 0000

A - (A>>1 & 0111....) - (A>>2 & 0011....) - (A>>3 & 0001....):
B:    0100 0001 0011 0010
      4    1    3    2
```

The next step is to combine the summation of each of those 4-bit groups into sums of 8-bit groups. To do this, we will use another bit trick. We will shift this number B to the right by 4 (B >> 4), and add that back to B. Then, we will apply a bit masking of 0000 1111 0000 1111 .... (in u64, this is 1085102592571150095u64) to yield the sums of bits in groups of 8, a number we will call C.

```text
B:    0100 0001 0011 0010
B>>4: 0000 0100 0001 0011
      0100 0101 0100 0101
      4    5    4    5

apply the bit mask
      0000 1111 0000 1111

C:    0000 0101 0000 0101
      0    5    0    5
```

At this point, `C` contains bit sums in eight-bit groups. The required result is the total number of bits in the original value.
Calculate `C` modulo 255 to get this result. The value 255 is equal to `2^8 - 1`.

For example, consider `1 0000 0001`. Modulo 256 gives 1, but modulo 255 gives 2.
The modulo 255 operation combines the bit sums from all eight-bit groups.

The following summary starts with a 64-bit integer `A`. It follows the `c_bitcount` closure in `verify.aleo`.

```text
let A = 64 unsigned bit integer
let B = A - (A>>1 & 8608480567731124087u64) - (A>>2 & 3689348814741910323u64) - (A>>3 & 1229782938247303441u64)
let C = (B - B>>4) & 1085102592571150095u64
bit count = C mod 255u64
```

### Adjacency Check

Use the ship position and its horizontal or vertical bitstring to determine if its bits are adjacent.
See the `c_adjacency_check` closure in `verify.aleo`.
A ship of length 2 has the horizontal bitstring `11`, or `3u64`.
Its vertical bitstring is `100000001`, or `257u64`.
If the ship starts at the bottom-right corner, its horizontal position bitstring is:

3u64

```text
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 1 1
```

Vertical ship placement:  
257u64

```text
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 1
0 0 0 0 0 0 0 1
```

If we move the ship to the left one column:  
Horizontal 6u64

```text
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 1 1 0
```

Vertical 514u64

```text
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 1 0
0 0 0 0 0 0 1 0
```

If we move the ship up one row:  
Horizontal 768u64

```text
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 1 1
0 0 0 0 0 0 0 0
```

Vertical 65792u64

```text
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 1
0 0 0 0 0 0 0 1
0 0 0 0 0 0 0 0
```

Each valid board position shifts the original bitstring by a power of 2.
Divide the ship position bitstring by the horizontal or vertical ship bitstring.
If the result is a power of 2, the ship's bits are adjacent.

To ensure that the remaining number is a power of 2, we can use a bit trick. See the bit trick for ensuring a bitstring is a power of 2 section.

The code has one additional step. Division can produce 0, and subtraction of 1 from 0 causes an underflow.
If division produces 0, the ship position is not valid. Set a value that cannot be a power of 2.

### Splitting a row or column

See the `c_horizontal_check` closure in `verify.aleo`. Assume that all bits are adjacent, as described in the adjacency check section.
The column check is direct. If a ship bitstring crosses columns, the adjacency division does not produce a power of 2.
Thus, the adjacency check rejects the position.

The row check is necessary because a bitstring across two rows can still have adjacent bits.
To simplify the check, calculate the 64-bit position bitstring modulo 255. This operation produces an eight-bit bitstring.
A valid position produces a valid eight-bit bitstring. An invalid position produces an invalid eight-bit bitstring.
For example:

```text
1 1 1 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
```

mod 255 = 11100000 (valid)

```text
0 0 0 0 0 0 0 1
1 1 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
```

mod 255 = 11000001 (invalid)

How do we know the 8 bit bitstring is valid or not? We can simply do an adjacency check, as before.

### Ensuring a bitstring is a power of 2

Any power of 2 will have a single bit flipped. If we subtract 1 from that number, it will result in a complementary bitstring that, bitwise-anded with the original, will always result in 0.

For example

```text
8:   1000
8-1: 0111
8&7: 0000 == 0

7:   0111
7-1: 0110
7&6: 0110 != 0
```

## Validating all ships together in a single board

Combine the valid ship position bitstrings with bitwise OR operators. See the `create_board` function in `verify.aleo`.
Then, count the bits on the complete board. Ships with lengths 5, 4, 3, and 2 must have a total of 14 bits.

## Ensure that players and boards cannot swap mid-game

Board states are represented with the board_state record. Each board has a flag indicating whether a game has been started with the board. This flag is set when offering a battleship game to an opponent, or accepting a battleship game from an opponent. Move records are created only in 3 ways:

1. Offering a battleship game creates a dummy move record that sets the two players to the addresses set in the board state record.
2. Accepting a Battleship game consumes the first dummy move record.
   The function verifies that the record and the accepting player's board contain the same players.
   Then, the function creates a new dummy move record with the same players.
3. Each play consumes a move record and creates the next move record.
   The function verifies that the move record and board contain the same players.
   It automatically puts these players in the next move record.

Moves from different boards can mix only when the same players start multiple games with each other.
If one player accepts only one game with an opponent, only one set of moves can use their board.

## Ensure that each player can only move once before the next player can move

A player must consume a move record to create the next move record. The record owner changes with each play.
Player A consumes a move record and creates a record that contains their fire coordinate. Player B owns the new record.
Player B must consume that record to create the next record, which Player A owns.

## Enforce valid moves

A valid fire coordinate has only one set bit in a `u64`. Use the power-of-2 check to verify this condition.
The coordinate must not be in the player's previous moves. `board.aleo::update_played_tiles` checks this condition.

To send a new move, call `main.aleo::play`. This function checks the opponent's fire coordinate on the current player's board.
The new move record tells the opponent if that coordinate was a hit or a miss.

## Winning the game

To check for a win, count the hits in the `hits_and_misses` field of your `board_state` record.
You win the game when this field contains 14 hits.
