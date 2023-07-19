program vote.aleo;

interface ProposalInfo:
    title as field;
    content as field;
    proposer as address;

record Proposal:
    owner as address.private;
    gates as u64.private;
    id as field.private;
    info as ProposalInfo.private;

record Ticket:
    owner as address.private;
    gates as u64.private;
    pid as field.private;

mapping proposals:
	key left as field.public;
	value right as ProposalInfo.public;

mapping tickets:
	key left as field.public;
	value right as u64.public;

mapping agree_votes:
	key left as field.public;
	value right as u64.public;

mapping disagree_votes:
	key left as field.public;
	value right as u64.public;

function propose:
    input r0 as ProposalInfo.public;
    assert.eq self.caller r0.proposer;
    hash.bhp256 r0.title into r1;
    cast self.caller 0u64 r1 r0 into r2 as Proposal.record;
    output r2 as Proposal.record;
    finalize r1;

finalize propose:
    input r0 as field.public;
    increment tickets[r0] by 0u64;

function new_ticket:
    input r0 as field.public;
    input r1 as address.public;
    cast r1 0u64 r0 into r2 as Ticket.record;
    output r2 as Ticket.record;
    finalize r0;

finalize new_ticket:
    input r0 as field.public;
    increment tickets[r0] by 1u64;

function agree:
    input r0 as Ticket.record;
    finalize r0.pid;

finalize agree:
    input r0 as field.public;
    increment agree_votes[r0] by 1u64;

function disagree:
    input r0 as Ticket.record;
    finalize r0.pid;

finalize disagree:
    input r0 as field.public;
    increment disagree_votes[r0] by 1u64;

