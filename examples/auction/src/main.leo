record Auction {
    owner: address,
    gates: u64,
    winning_bidder: address,
    winning_bid: u64
}

function main(current: Auction, bidder: address, bid: u64) -> Auction {
    if bid > current.winning_bid {
        return Auction {
            owner: current.owner,
            gates: current.gates,
            winning_bidder: bidder,
            winning_bid: bid
        };
    } else {
        return current;
    }
}
