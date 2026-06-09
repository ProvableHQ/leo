---
id: auction
title: A Private Auction using Leo
---

[general tags]: # "example, auction, record, program, assert"

**[Source Code](https://github.com/ProvableHQ/leo-examples/tree/main/auction)**

## Summary

A first-price sealed-bid auction (or blind auction) is a type of auction in which each participant submits a bid without knowing the bids of the other participants.
The bidder with the highest bid wins the auction.

In this model, there are two kinds of parties: the auctioneer and the bidders.

- **Bidder**: A participant in the auction.
- **Auctioneer**: The party responsible for conducting the auction.

We make following assumptions about the auction:

- The auctioneer is honest. That is, the auctioneer will resolve **all** bids in the order they are received. The auctioneer will not tamper with the bids.
- There is no limit to the number of bids.
- The auctioneer knows the identity of all bidders, but bidders do not necessarily know the identity of other bidders.

Under this model, we require that:

- Bidders do not learn any information about the value of other bids.

## Auction Flow

The auction is conducted in a series of stages.

- **Bidding**: In the bidding stage, bidders submit bids to the auctioneer. They do so by invoking the `place_bid` function.
- **Resolution**: In the resolution stage, the auctioneer resolves the bids in the order they were received. The auctioneer does so by invoking the `resolve` function. The resolution process produces a single winning bid.
- **Finishing**: In this stage, the auctioneer finishes the auction by invoking the `finish` function. This function returns the winning bid to the bidder, which the bidder can then use to claim the item.

## Language Features and Concepts

- `record` declarations
- `assert_eq`
- record ownership

## How to Run

Follow the [Leo Installation Instructions](https://docs.leo-lang.org/getting_started/installation).

This auction program can be run using the following bash script. Locally, it will execute Leo program functions to conduct, bid, and close a three party auction.

```bash
cd leo/examples/auction
./run.sh
```

The `.env` file contains a private key and address. This is the account that will be used to sign transactions and is checked for record ownership. When executing programs as different parties, be sure to set the `private_key` field in `.env` to the appropriate value. You can check out how we've set things up in `./run.sh` for a full example of how to run the program as different parties.

## Walkthrough

- [Step 0: Initializing the Auction](#step0)
- [Step 1: The First Bid](#step1)
- [Step 2: The Second Bid](#step2)
- [Step 3: Select the Winner](#step3)

## <a id="step0"></a> Step 0: Initializing the Auction

The three parties we'll be emulating are as follows:

```markdown
Bidder 1 Private Key:  
APrivateKey1zkpG9Af9z5Ha4ejVyMCqVFXRKknSm8L1ELEwcc4htk9YhVK
Bidder 1 Address:
aleo1yzlta2q5h8t0fqe0v6dyh9mtv4aggd53fgzr068jvplqhvqsnvzq7pj2ke

Bidder 2 Private Key:
APrivateKey1zkpAFshdsj2EqQzXh5zHceDapFWVCwR6wMCJFfkLYRKupug
Bidder 2 Address:
aleo1esqchvevwn7n5p84e735w4dtwt2hdtu4dpguwgwy94tsxm2p7qpqmlrta4

Auctioneer Private Key:
APrivateKey1zkp5wvamYgK3WCAdpBQxZqQX8XnuN2u11Y6QprZTriVwZVc
Auctioneer Address:
aleo1fxs9s0w97lmkwlcmgn0z3nuxufdee5yck9wqrs0umevp7qs0sg9q5xxxzh
```

## <a id="step1"></a> Step 1: The First Bid

Have the first bidder place a bid of 10.

Swap in the private key and address of the first bidder to `.env`.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkpG9Af9z5Ha4ejVyMCqVFXRKknSm8L1ELEwcc4htk9YhVK
ENDPOINT=https://localhost:3030
" > .env
```

Call the `place_bid` program function with bidder 1 and `10u64` arguments.

```bash
leo run place_bid aleo1yzlta2q5h8t0fqe0v6dyh9mtv4aggd53fgzr068jvplqhvqsnvzq7pj2ke 10u64
```

Output:

```bash
 • {
  owner: aleo1yzlta2q5h8t0fqe0v6dyh9mtv4aggd53fgzr068jvplqhvqsnvzq7pj2ke.private,
  bidder: aleo1yzlta2q5h8t0fqe0v6dyh9mtv4aggd53fgzr068jvplqhvqsnvzq7pj2ke.private,
  amount: 10u64.private,
  is_winner: false.private,
  _nonce: 4668394794828730542675887906815309351994017139223602571716627453741502624516group.public
}
```

## <a id="step2"></a> Step 2: The Second Bid

Have the second bidder place a bid of 90.

Swap in the private key of the second bidder to `.env`.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkpAFshdsj2EqQzXh5zHceDapFWVCwR6wMCJFfkLYRKupug
ENDPOINT=https://localhost:3030
" > .env
```

Call the `place_bid` program function with bidder 2 and `90u64` arguments.

```bash
leo run place_bid aleo1esqchvevwn7n5p84e735w4dtwt2hdtu4dpguwgwy94tsxm2p7qpqmlrta4 90u64
```

Output:

```bash
 • {
  owner: aleo1esqchvevwn7n5p84e735w4dtwt2hdtu4dpguwgwy94tsxm2p7qpqmlrta4.private,
  bidder: aleo1esqchvevwn7n5p84e735w4dtwt2hdtu4dpguwgwy94tsxm2p7qpqmlrta4.private,
  amount: 90u64.private,
  is_winner: false.private,
  _nonce: 5952811863753971450641238938606857357746712138665944763541786901326522216736group.public
}
```

## <a id="step3"></a> Step 3: Select the Winner

Have the auctioneer select the winning bid.

Swap in the private key of the auctioneer to `.env`.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkp5wvamYgK3WCAdpBQxZqQX8XnuN2u11Y6QprZTriVwZVc
ENDPOINT=https://localhost:3030
" > .env
```

Provide the two `Bid` records as input to the `resolve` function.

```bash
leo run resolve "{
    owner: aleo1fxs9s0w97lmkwlcmgn0z3nuxufdee5yck9wqrs0umevp7qs0sg9q5xxxzh.private,
    bidder: aleo1yzlta2q5h8t0fqe0v6dyh9mtv4aggd53fgzr068jvplqhvqsnvzq7pj2ke.private,
    amount: 10u64.private,
    is_winner: false.private,
    _nonce: 4668394794828730542675887906815309351994017139223602571716627453741502624516group.public
}" "{
    owner: aleo1fxs9s0w97lmkwlcmgn0z3nuxufdee5yck9wqrs0umevp7qs0sg9q5xxxzh.private,
    bidder: aleo1esqchvevwn7n5p84e735w4dtwt2hdtu4dpguwgwy94tsxm2p7qpqmlrta4.private,
    amount: 90u64.private,
    is_winner: false.private,
    _nonce: 5952811863753971450641238938606857357746712138665944763541786901326522216736group.public
}"
```

## <a id="step4"></a> Step 4: Finish the Auction

Call the `finish` function with the winning `Bid` record.

```bash
leo run finish "{
    owner: aleo1fxs9s0w97lmkwlcmgn0z3nuxufdee5yck9wqrs0umevp7qs0sg9q5xxxzh.private,
    bidder: aleo1esqchvevwn7n5p84e735w4dtwt2hdtu4dpguwgwy94tsxm2p7qpqmlrta4.private,
    amount: 90u64.private,
    is_winner: false.private,
    _nonce: 5952811863753971450641238938606857357746712138665944763541786901326522216736group.public
}"
```

Congratulations! You've run a private auction. We recommend going to [provable.tools](https://provable.tools) to generate new accounts and trying the same commands with those addresses.
