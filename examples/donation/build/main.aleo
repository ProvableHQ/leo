program donation.aleo;

record user_record:
    owner as address.private;
    amount as u64.private;
    id_assigned as boolean.private;
    user as address.private;

record Token:
    owner as address.private;
    amount as u64.private;


mapping balances:
	key as field.public;
	value as u64.public;

closure only_owner:
    input r0 as address;
    assert.eq self.caller r0;
    output r0 as address;


function donate:
    input r0 as Token.record;
    input r1 as u64.private;
    sub r0.amount r1 into r2;
    cast r0.owner r2 into r3 as Token.record;
    hash.bhp256 r0.owner into r4 as field;
    async donate r4 r1 into r5;
    output r3 as Token.record;
    output r5 as donation.aleo/donate.future;

finalize donate:
    input r0 as field.public;
    input r1 as u64.public;
    get.or_use balances[r0] 0u64 into r2;
    add r2 r1 into r3;
    set r3 into balances[r0];


function ajo:
    input r0 as address.private;
    input r1 as u64.private;
    assert.eq self.caller r0;
    cast aleo1fxs9s0w97lmkwlcmgn0z3nuxufdee5yck9wqrs0umevp7qs0sg9q5xxxzh r1 true r0 into r2 as user_record.record;
    output r2 as user_record.record;
