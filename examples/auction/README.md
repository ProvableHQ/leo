<!-- # 🏛️ Blind Auction -->

[//]: # (<img alt="workshop/auction" width="1412" src="../.resources/auction.png">)

A first-price sealed-bid auction in Leo.

## Summary

A first-price sealed-bid auction (or blind auction) is a type of auction in which each participant submits a bid without knowing the bids of the other participants. 
The bidder with the highest bid wins the auction.

In this model, there are two parties: the auctioneer and the bidders.
- **Bidder**: A participant in the auction.
- **Auctioneer**: The party responsible for conducting the auction.

We make the following assumptions about the auction:
- The auctioneer is honest. That is, the auctioneer will resolve **all** bids in the order they are received. The auctioneer will not tamper with the bids.
- There is no limit to the number of bids.
- The auctioneer knows the identity of all bidders, but bidders do not necessarily know the identity of other bidders.

Under this model, we require that:
- Bidders do not learn any information about the value of other bids.

### Auction Flow
The auction is conducted in a series of stages.
- **Bidding**: In the bidding stage, bidders submit bids to the auctioneer. They do so by invoking the `place_bid` function.
- **Resolution**:  In the resolution stage, the auctioneer resolves the bids in the order they were received. The auctioneer does so by invoking the `resolve` function. The resolution process produces a single winning bid.
- **Finishing**: In this stage, the auctioneer finishes the auction by invoking the `finish` function. This function returns the winning bid to the bidder, which the bidder can then use to claim the item.


## Language Features and Concepts
- `record` declarations
- `assert_eq`
- record ownership

## Running the Program

Leo provides users with a command line interface for compiling and running Leo programs.

### Configuring Accounts
The `.env` file contains a private key. 
This is the account that will be used to sign transactions and is checked for record ownership.
When executing programs as different parties, be sure to set the `PRIVATE_KEY` field in `.env` to the appropriate values.
See `./run.sh` for an example of how to run the program as different parties.


The [Aleo SDK](https://github.com/ProvableHQ/leo/tree/mainnet) provides an interface for generating new accounts.
To generate a new account, navigate to [provable.tools](https://provable.tools).


### Providing inputs via the command line.
```bash
leo run <function_name> <input_1> <input_2> ...
```
See `./run.sh` for an example.


