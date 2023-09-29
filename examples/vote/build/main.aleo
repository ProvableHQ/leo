program vote.aleo;

struct ProposalInfo:
    title as field;
    content as field;
    proposer as address;

record Proposal:
    owner as address.private;
    id as field.private;
    info as ProposalInfo.private;

record Ticket:
    owner as address.private;
    pid as field.private;


mapping proposals:
	key as field.public;
	value as ProposalInfo.public;


mapping tickets:
	key as field.public;
	value as u64.public;


mapping agree_votes:
	key as field.public;
	value as u64.public;


mapping disagree_votes:
	key as field.public;
	value as u64.public;

function propose:
    input r0 as ProposalInfo.public;
    assert.eq self.caller r0.proposer;
    hash.bhp256 r0.title into r1 as field;
    cast self.caller r1 r0 into r2 as Proposal.record;
    async propose r1 into r3;    output r2 as Proposal.record;
    output r3 as vote.aleo/propose.future;

finalize propose:
    input r0 as field.public;
    set 0u64 into tickets[r0];


function new_ticket:
    input r0 as field.public;
    input r1 as address.public;
    cast r1 r0 into r2 as Ticket.record;
    async new_ticket r0 into r3;    output r2 as Ticket.record;
    output r3 as vote.aleo/new_ticket.future;

finalize new_ticket:
    input r0 as field.public;
    get.or_use tickets[r0] 0u64 into r1;
    add r1 1u64 into r2;
    set r2 into tickets[r0];


function agree:
    input r0 as Ticket.record;
    async agree r0.pid into r1;    output r1 as vote.aleo/agree.future;

finalize agree:
    input r0 as field.public;
    get.or_use agree_votes[r0] 0u64 into r1;
    add r1 1u64 into r2;
    set r2 into agree_votes[r0];


function disagree:
    input r0 as Ticket.record;
    async disagree r0.pid into r1;    output r1 as vote.aleo/disagree.future;

finalize disagree:
    input r0 as field.public;
    get.or_use disagree_votes[r0] 0u64 into r1;
    add r1 1u64 into r2;
    set r2 into disagree_votes[r0];
