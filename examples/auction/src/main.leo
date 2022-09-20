// This example is suggestive of an auction smart contract,
// but it is much simpler of a full auction smart contract.

// This is a record that contains the current state of the auction,
// which in this example just consists of
// the address of the currently winning bidder
// and the currently winning amount.
// The owner and gates fields are required in a record;
// the owner address in general differs from the current auction winner:
// the owner is the one running the auction itself.

record Auction {
    owner: address,
    gates: u64,
    winning_bidder: address,
    winning_bid: u64
}

// This function realizes the checking and possible incorporation of a new bid.
// Unlike a traditional auction, where the currently winning bid is known,
// and only higher bids would be normally brought up in the room,
// in a zero-knowledge context presumably bidding is blind,
// i.e. a bidder does not know the currently highest bid.
// Thus, this function represents a possible update of the auction state
// with a new bid:
// if the amount exceeds the currently winning one,
// the record is updated with the new bid and the new bidder's address;
// the latter is also passed as input.
// Again, this is just a simple example;
// in a serious auction smart contract,
// there will be some additional checks,
// and presumably ways to tie bidder and amount (e.g. commitments).
//
// In order to run this function, an Auction record must be passed to it.
// This is done in the input file, inputs/auction.in.
// Note that, when building that record,
// the owner address must be the same as the one in the program.json file.

@program
function main(current: Auction, bidder: address, bid: u64) -> Auction {
    let winning_bidder: address = current.winning_bidder;
    let winning_bid: u64 = current.winning_bid;
    if bid > current.winning_bid {
        winning_bidder = bidder;
        winning_bid = bid;
    }
    let new: Auction = Auction {
        owner: current.owner,
        gates: current.gates,
        winning_bidder: winning_bidder,
        winning_bid: winning_bid
    };
    return new;
}
