// This record defines a bid in the auction.
record Bid {
    owner: address,
    gates: u64,
    bidder: address,
    amount: u64,
    is_winner: bool,
}

// This function creates a new bid.
// This function checks that `bidder` invoked the function.
// Note that the owner of the record is set to the entity responsible for running the auction.
//   The entity's address is aleo1fxs9s0w97lmkwlcmgn0z3nuxufdee5yck9wqrs0umevp7qs0sg9q5xxxzh.
//   This is necessary to allow the entity running the auction to process bids.
@program
function place_bid(bidder: address, amount: u64) -> Bid {
    console.assert_eq(self.caller, bidder);
    return Bid {
        owner: aleo1fxs9s0w97lmkwlcmgn0z3nuxufdee5yck9wqrs0umevp7qs0sg9q5xxxzh,
        gates: 0u64,
        bidder: bidder,
        amount: amount,
        is_winner: false,
    };
}

// This function consumes two bids, selecting the winning one. In the event of a tie, the first bid is selected.
// This function should be invoked once the bidding period has ended.
// This function can only be called by the entity running the auction.
@program
function resolve(first: Bid, second: Bid) -> Bid {
    console.assert_eq(self.caller, aleo1fxs9s0w97lmkwlcmgn0z3nuxufdee5yck9wqrs0umevp7qs0sg9q5xxxzh);
    if (first.amount >= second.amount) {
        return first;
    } else {
        return second;
    }
}

// This function returns ownership of the `Bid` record to the original bidder.
// This function should be called after all bids have been resolved.
// This function can only be called by the entity running the auction.
@program
function finish(bid: Bid) -> Bid {
    console.assert_eq(self.caller, aleo1fxs9s0w97lmkwlcmgn0z3nuxufdee5yck9wqrs0umevp7qs0sg9q5xxxzh);
    return Bid {
        owner: bid.bidder,
        gates: bid.gates,
        bidder: bid.bidder,
        amount: bid.amount,
        is_winner: true,
    };
}
