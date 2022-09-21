// The 'vote.leo' program.

// Proposal details
circuit ProposalInfo {
    title: field,
    content: field,
    proposer: address,
}

// Proposal record records proposal info publicly
record Proposal {
    owner: address,
    gates: u64,
    id: field,
    info: ProposalInfo,
}

// Save proposal info in public storage.
mapping proposals: field => ProposalInfo;

// Privacy tickets to vote
record Ticket {
    owner: address,
    gates: u64,
    pid: field,
}

// Count the total tickets issued for each proposal
mapping tickets: field => u64;

mapping agree_votes: field => u64;

mapping disagree_votes: field => u64;

// Propose a new proposal to vote on.
@program
function propose(public info: ProposalInfo) -> Proposal {
    // Authenticate proposer.
    console.assert_eq(self.caller, info.proposer);

    // Generate a new proposal id.
    let id: field = BHP256::hash(info.title);

    // Finalize the proposal id.
    async finalize(id);

    // Return a new record for the proposal.
    return Proposal {
        owner: self.caller,
        gates: 0u64,
        id,
        info,
    };
}
// Create a new proposal in the "tickets" mapping.
finalize propose(public id: field) {
    increment(tickets, id, 0u64);
}

// Create a new ticket to vote with.
@program
function new_ticket(
    public pid: field,
    public voter: address,
) -> Ticket {
    // Finalize the proposal id for the ticket.
    async finalize(pid);

    return Ticket {
        owner: voter,
        gates: 0u64,
        pid,
    };
}
// Create a new ticket on a proposal in the "tickets" mapping.
finalize new_ticket(public pid: field) {
    increment(tickets, pid, 1u64);
}

// Vote privately to agree with a proposal.
@program
function agree(ticket: Ticket) {
    // Finalize this vote.
    async finalize(ticket.pid);
}
finalize agree(public pid: field) {
    // Publicly increment the number of agree votes.
    increment(agree_votes, pid, 1u64);
}

// Vote privately to disagree with a proposal.
@program
function disagree(ticket: Ticket) {
    // Finalize this vote.
    async finalize(ticket.pid);
}
finalize disagree(pid: field) {
    // Publicly increment the number of disagree votes.
    increment(disagree_votes, pid, 1u64);
}
