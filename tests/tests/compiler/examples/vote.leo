
// The 'vote.leo' program.


// Proposal details
struct ProposalInfo {
    title: field,
    content: field,
    proposer: address,
}

// Create a new proposal in the "tickets" mapping.
final fn finalize_propose(public id: field) {
    Mapping::set(tickets, id, 0u64);
}

// Create a new ticket on a proposal in the "tickets" mapping.
final fn finalize_new_ticket(public pid: field) {
    let current: u64 = Mapping::get_or_use(tickets, pid, 0u64);
    Mapping::set(tickets, pid, current + 1u64);
}

final fn finalize_agree(public pid: field) {
    // Publicly increment the number of agree votes.
    let current: u64 = Mapping::get_or_use(agree_votes, pid, 0u64);
    Mapping::set(agree_votes, pid, current + 1u64);
}

final fn finalize_disagree(pid: field) {
    // Publicly increment the number of disagree votes.
    let current: u64 = Mapping::get_or_use(disagree_votes, pid, 0u64);
    Mapping::set(disagree_votes, pid, current + 1u64);
}

program vote.aleo {
    // Save proposal info in public storage.
    mapping proposals: field => ProposalInfo;

    // Proposal record records proposal info publicly
    record Proposal {
        owner: address,
        id: field,
        info: ProposalInfo,
    }

    // Privacy tickets to vote
    record Ticket {
        owner: address,
        pid: field,
    }

    // Count the total tickets issued for each proposal
    mapping tickets: field => u64;

    mapping agree_votes: field => u64;

    mapping disagree_votes: field => u64;

    // Propose a new proposal to vote on.
    fn propose(public info: ProposalInfo) -> (Proposal, Final) {
        // Authenticate proposer.
        assert_eq(self.signer, info.proposer);

        // Generate a new proposal id.
        let id: field = BHP256::hash_to_field(info.title);


        // Return a new record for the proposal.
        // Finalize the proposal id.
        return (Proposal { owner: self.signer, id, info }, final { finalize_propose(id); });
    }
    // Create a new ticket to vote with.
    fn new_ticket(
        public pid: field,
        public voter: address,
    ) -> (Ticket, Final) {

        // Finalize the proposal id for the ticket.
        return (Ticket { owner: voter, pid }, final { finalize_new_ticket(pid); });
    }
    // Vote privately to agree with a proposal.
    fn agree(ticket: Ticket) -> Final {
        // Finalize this vote.
        let pid = ticket.pid;
        return final { finalize_agree(pid); };
    }
    // Vote privately to disagree with a proposal.
    fn disagree(ticket: Ticket) -> Final {
        // Finalize this vote.
        let pid = ticket.pid;
        return final { finalize_disagree(pid); };
    }
}
