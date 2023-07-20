program lottery.aleo;

record Ticket:
    owner as address.private;


mapping num_winners:
	key left as u8.public;
	value right as u8.public;

function play:
    cast self.caller into r0 as Ticket.record;
    output r0 as Ticket.record;

    finalize;

finalize play:
    lte block.height 1000u32 into r0;
    assert.eq r0 true;
    rand.chacha into r1 as boolean;
    assert.eq r1 true;
    get.or_use num_winners[0u8] 0u8 into r2;
    lt r2 5u8 into r3;
    assert.eq r3 true;
    add r2 1u8 into r4;
    set r4 into num_winners[0u8];
