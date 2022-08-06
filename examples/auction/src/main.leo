record Auction {
    owner: address,
    gates: u64,
    winning_bidder: address,
    winning_bid: u64
}

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
