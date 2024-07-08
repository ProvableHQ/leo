#!/bin/bash
# First check that Leo is installed.
if ! command -v leo &> /dev/null
then
    echo "leo is not installed."
    exit
fi

# The private key and address of Alice.
# Swap these into program.json, when running transactions as the first bidder.
# NETWORK=testnet
# PRIVATE_KEY=APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH

# The private key and address of Bob.
# Swap these into program.json, when running transactions as the second bidder.
# NETWORK=testnet
# PRIVATE_KEY=APrivateKey1zkp2RWGDcde3efb89rjhME1VYA8QMxcxep5DShNBR6n8Yjh

# Swap in the private key of Alice.
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH
ENDPOINT=https://localhost:3030
" > .env

# Publicly mint 100 tokens for Alice.
echo "
###############################################################################
########                                                               ########
########     STEP 1: Publicly mint 100 tokens for Alice                ########
########                                                               ########
########           -----------------------------------------           ########
########           |            PUBLIC BALANCES            |           ########
########           -----------------------------------------           ########
########           -----------------------------------------           ########
########           |        Alice    |         100         |           ########
########           -----------------------------------------           ########
########           |        Bob      |          0          |           ########
########           -----------------------------------------           ########
########                                                               ########
########           -----------------------------------------           ########
########           |            PRIVATE BALANCES           |           ########
########           -----------------------------------------           ########
########           -----------------------------------------           ########
########           |        Alice    |          0          |           ########
########           -----------------------------------------           ########
########           |        Bob      |          0          |           ########
########           -----------------------------------------           ########
########                                                               ########
###############################################################################
"
leo run mint_public aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px 100u64

# Swap in the private key of Bob.
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkp2RWGDcde3efb89rjhME1VYA8QMxcxep5DShNBR6n8Yjh
ENDPOINT=https://localhost:3030
" > .env

# Privately mint 100 tokens for Bob.
echo "
###############################################################################
########                                                               ########
########     STEP 2: Privately mint 100 tokens for Bob                 ########
########                                                               ########
########           -----------------------------------------           ########
########           |            PUBLIC BALANCES            |           ########
########           -----------------------------------------           ########
########           -----------------------------------------           ########
########           |        Alice    |         100         |           ########
########           -----------------------------------------           ########
########           |        Bob      |          0          |           ########
########           -----------------------------------------           ########
########                                                               ########
########           -----------------------------------------           ########
########           |            PRIVATE BALANCES           |           ########
########           -----------------------------------------           ########
########           -----------------------------------------           ########
########           |        Alice    |          0          |           ########
########           -----------------------------------------           ########
########           |        Bob      |         100         |           ########
########           -----------------------------------------           ########
########                                                               ########
###############################################################################
"
leo run mint_private aleo1s3ws5tra87fjycnjrwsjcrnw2qxr8jfqqdugnf0xzqqw29q9m5pqem2u4t 100u64

# Swap in the private key of Alice.
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH
ENDPOINT=https://localhost:3030
" > .env

# Publicly transfer 10 tokens from Alice to Bob.
echo "
###############################################################################
########                                                               ########
########     STEP 3: Publicly transfer 10 tokens from Alice to Bob     ########
########                                                               ########
########           -----------------------------------------           ########
########           |            PUBLIC BALANCES            |           ########
########           -----------------------------------------           ########
########           -----------------------------------------           ########
########           |        Alice    |          90         |           ########
########           -----------------------------------------           ########
########           |        Bob      |          10         |           ########
########           -----------------------------------------           ########
########                                                               ########
########           -----------------------------------------           ########
########           |            PRIVATE BALANCES           |           ########
########           -----------------------------------------           ########
########           -----------------------------------------           ########
########           |        Alice    |          0          |           ########
########           -----------------------------------------           ########
########           |        Bob      |         100         |           ########
########           -----------------------------------------           ########
########                                                               ########
###############################################################################
"
leo run transfer_public aleo1s3ws5tra87fjycnjrwsjcrnw2qxr8jfqqdugnf0xzqqw29q9m5pqem2u4t 10u64

# Swap in the private key of Bob.
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkp2RWGDcde3efb89rjhME1VYA8QMxcxep5DShNBR6n8Yjh
ENDPOINT=https://localhost:3030
" > .env

# Privately transfer 20 tokens from Bob to Alice.
echo "
###############################################################################
########                                                               ########
########     STEP 4: Privately transfer 20 tokens from Bob to Alice    ########
########                                                               ########
########           -----------------------------------------           ########
########           |            PUBLIC BALANCES            |           ########
########           -----------------------------------------           ########
########           -----------------------------------------           ########
########           |        Alice    |          90         |           ########
########           -----------------------------------------           ########
########           |        Bob      |          10         |           ########
########           -----------------------------------------           ########
########                                                               ########
########           -----------------------------------------           ########
########           |            PRIVATE BALANCES           |           ########
########           -----------------------------------------           ########
########           -----------------------------------------           ########
########           |        Alice    |          20         |           ########
########           -----------------------------------------           ########
########           |        Bob      |          80         |           ########
########           -----------------------------------------           ########
########                                                               ########
###############################################################################
"
leo run transfer_private "{
        owner: aleo1s3ws5tra87fjycnjrwsjcrnw2qxr8jfqqdugnf0xzqqw29q9m5pqem2u4t.private,
        amount: 100u64.private,
        _nonce: 6586771265379155927089644749305420610382723873232320906747954786091923851913group.public
    }" aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px 20u64

# Swap in the private key of Alice.
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH
ENDPOINT=https://localhost:3030
" > .env

# Convert 30 public tokens from Alice into 30 private tokens for Bob.
echo "
###############################################################################
########                                                               ########
########     STEP 5: Convert 30 public tokens from Alice into 30       ########
########             private tokens for Bob.                           ########
########                                                               ########
########           -----------------------------------------           ########
########           |            PUBLIC BALANCES            |           ########
########           -----------------------------------------           ########
########           -----------------------------------------           ########
########           |        Alice    |          60         |           ########
########           -----------------------------------------           ########
########           |        Bob      |          10         |           ########
########           -----------------------------------------           ########
########                                                               ########
########           -----------------------------------------           ########
########           |            PRIVATE BALANCES           |           ########
########           -----------------------------------------           ########
########           -----------------------------------------           ########
########           |        Alice    |          20         |           ########
########           -----------------------------------------           ########
########           |        Bob      |         110         |           ########
########           -----------------------------------------           ########
########                                                               ########
###############################################################################
"
leo run transfer_public_to_private aleo1s3ws5tra87fjycnjrwsjcrnw2qxr8jfqqdugnf0xzqqw29q9m5pqem2u4t 30u64

# Swap in the private key of Bob.
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkp2RWGDcde3efb89rjhME1VYA8QMxcxep5DShNBR6n8Yjh
ENDPOINT=https://localhost:3030
" > .env

# Convert 40 private tokens from Bob into 40 public tokens for Alice.
echo "
###############################################################################
########                                                               ########
########     STEP 6: Convert 40 private tokens from Bob into 40        ########
########             public tokens for Alice.                          ########
########                                                               ########
########           -----------------------------------------           ########
########           |            PUBLIC BALANCES            |           ########
########           -----------------------------------------           ########
########           -----------------------------------------           ########
########           |        Alice    |         100         |           ########
########           -----------------------------------------           ########
########           |        Bob      |          10         |           ########
########           -----------------------------------------           ########
########                                                               ########
########           -----------------------------------------           ########
########           |            PRIVATE BALANCES           |           ########
########           -----------------------------------------           ########
########           -----------------------------------------           ########
########           |        Alice    |          20         |           ########
########           -----------------------------------------           ########
########           |        Bob      |          70         |           ########
########           -----------------------------------------           ########
########                                                               ########
###############################################################################
"
leo run transfer_private_to_public "{
        owner: aleo1s3ws5tra87fjycnjrwsjcrnw2qxr8jfqqdugnf0xzqqw29q9m5pqem2u4t.private,
        amount: 80u64.private,
        _nonce: 1852830456042139988098466781381363679605019151318121788109768539956661608520group.public
    }" aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px 40u64


# Swap in the private key of Alice.
# This is done to ensure that program.json is the same after every execution of ./run.sh.
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH
ENDPOINT=https://localhost:3030
" > .env
