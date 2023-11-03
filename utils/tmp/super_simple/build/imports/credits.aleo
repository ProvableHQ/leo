program credits.aleo;

mapping committee:
    key as address.public;
    value as committee_state.public;

struct committee_state:
    microcredits as u64;
    is_open as boolean;

mapping bonded:
    key as address.public;
    value as bond_state.public;

struct bond_state:
    validator as address;
    microcredits as u64;

mapping unbonding:
    key as address.public;
    value as unbond_state.public;

struct unbond_state:
    microcredits as u64;
    height as u32;

mapping account:
    key as address.public;
    value as u64.public;

record credits:
    owner as address.private;
    microcredits as u64.private;

function bond_public:
    input r0 as address.public;
    input r1 as u64.public;
    gte r1 1000000u64 into r2;
    assert.eq r2 true ;
    async bond_public self.caller r0 r1 into r3;
    output r3 as credits.aleo/bond_public.future;

finalize bond_public:
    input r0 as address.public;
    input r1 as address.public;
    input r2 as u64.public;
    is.eq r0 r1 into r3;
    branch.eq r3 true to bond_validator;
    branch.eq r3 false to bond_delegator;
    position bond_validator;
    cast 0u64 true into r4 as committee_state;
    get.or_use committee[r0] r4 into r5;
    assert.eq r5.is_open true ;
    add r5.microcredits r2 into r6;
    cast r6 r5.is_open into r7 as committee_state;
    cast r1 0u64 into r8 as bond_state;
    get.or_use bonded[r0] r8 into r9;
    assert.eq r9.validator r1 ;
    add r9.microcredits r2 into r10;
    gte r10 1000000000000u64 into r11;
    assert.eq r11 true ;
    cast r1 r10 into r12 as bond_state;
    get account[r0] into r13;
    sub r13 r2 into r14;
    set r7 into committee[r0];
    set r12 into bonded[r0];
    set r14 into account[r0];
    branch.eq true true to end;
    position bond_delegator;
    contains committee[r0] into r15;
    assert.eq r15 false ;
    get committee[r1] into r16;
    assert.eq r16.is_open true ;
    add r16.microcredits r2 into r17;
    cast r17 r16.is_open into r18 as committee_state;
    cast r1 0u64 into r19 as bond_state;
    get.or_use bonded[r0] r19 into r20;
    assert.eq r20.validator r1 ;
    add r20.microcredits r2 into r21;
    gte r21 10000000u64 into r22;
    assert.eq r22 true ;
    cast r1 r21 into r23 as bond_state;
    get account[r0] into r24;
    sub r24 r2 into r25;
    set r18 into committee[r1];
    set r23 into bonded[r0];
    set r25 into account[r0];
    position end;

function unbond_public:
    input r0 as u64.public;
    async unbond_public self.caller r0 into r1;
    output r1 as credits.aleo/unbond_public.future;

finalize unbond_public:
    input r0 as address.public;
    input r1 as u64.public;
    cast 0u64 0u32 into r2 as unbond_state;
    get.or_use unbonding[r0] r2 into r3;
    add block.height 360u32 into r4;
    contains committee[r0] into r5;
    branch.eq r5 true to unbond_validator;
    branch.eq r5 false to unbond_delegator;
    position unbond_validator;
    get committee[r0] into r6;
    sub r6.microcredits r1 into r7;
    get bonded[r0] into r8;
    assert.eq r8.validator r0 ;
    sub r8.microcredits r1 into r9;
    gte r9 1000000000000u64 into r10;
    branch.eq r10 true to decrement_validator;
    branch.eq r10 false to remove_validator;
    position decrement_validator;
    cast r7 r6.is_open into r11 as committee_state;
    set r11 into committee[r0];
    cast r0 r9 into r12 as bond_state;
    set r12 into bonded[r0];
    add r3.microcredits r1 into r13;
    cast r13 r4 into r14 as unbond_state;
    set r14 into unbonding[r0];
    branch.eq true true to end;
    position remove_validator;
    assert.eq r6.microcredits r8.microcredits ;
    remove committee[r0];
    remove bonded[r0];
    add r3.microcredits r8.microcredits into r15;
    cast r15 r4 into r16 as unbond_state;
    set r16 into unbonding[r0];
    branch.eq true true to end;
    position unbond_delegator;
    get bonded[r0] into r17;
    sub r17.microcredits r1 into r18;
    gte r18 10000000u64 into r19;
    branch.eq r19 true to decrement_delegator;
    branch.eq r19 false to remove_delegator;
    position decrement_delegator;
    get committee[r17.validator] into r20;
    sub r20.microcredits r1 into r21;
    cast r21 r20.is_open into r22 as committee_state;
    set r22 into committee[r17.validator];
    cast r17.validator r18 into r23 as bond_state;
    set r23 into bonded[r0];
    add r3.microcredits r1 into r24;
    cast r24 r4 into r25 as unbond_state;
    set r25 into unbonding[r0];
    branch.eq true true to end;
    position remove_delegator;
    get committee[r17.validator] into r26;
    sub r26.microcredits r17.microcredits into r27;
    cast r27 r26.is_open into r28 as committee_state;
    set r28 into committee[r17.validator];
    remove bonded[r0];
    add r3.microcredits r17.microcredits into r29;
    cast r29 r4 into r30 as unbond_state;
    set r30 into unbonding[r0];
    position end;

function unbond_delegator_as_validator:
    input r0 as address.public;
    async unbond_delegator_as_validator self.caller r0 into r1;
    output r1 as credits.aleo/unbond_delegator_as_validator.future;

finalize unbond_delegator_as_validator:
    input r0 as address.public;
    input r1 as address.public;
    get committee[r0] into r2;
    assert.eq r2.is_open false ;
    contains committee[r1] into r3;
    assert.eq r3 false ;
    get bonded[r1] into r4;
    assert.eq r4.validator r0 ;
    sub r2.microcredits r4.microcredits into r5;
    cast r5 r2.is_open into r6 as committee_state;
    cast 0u64 0u32 into r7 as unbond_state;
    get.or_use unbonding[r1] r7 into r8;
    add r8.microcredits r4.microcredits into r9;
    add block.height 360u32 into r10;
    cast r9 r10 into r11 as unbond_state;
    set r6 into committee[r0];
    remove bonded[r1];
    set r11 into unbonding[r1];

function claim_unbond_public:
    async claim_unbond_public self.caller into r0;
    output r0 as credits.aleo/claim_unbond_public.future;

finalize claim_unbond_public:
    input r0 as address.public;
    get unbonding[r0] into r1;
    gte block.height r1.height into r2;
    assert.eq r2 true ;
    get.or_use account[r0] 0u64 into r3;
    add r1.microcredits r3 into r4;
    set r4 into account[r0];
    remove unbonding[r0];

function set_validator_state:
    input r0 as boolean.public;
    async set_validator_state self.caller r0 into r1;
    output r1 as credits.aleo/set_validator_state.future;

finalize set_validator_state:
    input r0 as address.public;
    input r1 as boolean.public;
    get committee[r0] into r2;
    cast r2.microcredits r1 into r3 as committee_state;
    set r3 into committee[r0];

function transfer_public:
    input r0 as address.public;
    input r1 as u64.public;
    async transfer_public self.caller r0 r1 into r2;
    output r2 as credits.aleo/transfer_public.future;

finalize transfer_public:
    input r0 as address.public;
    input r1 as address.public;
    input r2 as u64.public;
    get account[r0] into r3;
    sub r3 r2 into r4;
    set r4 into account[r0];
    get.or_use account[r1] 0u64 into r5;
    add r5 r2 into r6;
    set r6 into account[r1];

function transfer_private:
    input r0 as credits.record;
    input r1 as address.private;
    input r2 as u64.private;
    sub r0.microcredits r2 into r3;
    cast r1 r2 into r4 as credits.record;
    cast r0.owner r3 into r5 as credits.record;
    output r4 as credits.record;
    output r5 as credits.record;

function transfer_private_to_public:
    input r0 as credits.record;
    input r1 as address.public;
    input r2 as u64.public;
    sub r0.microcredits r2 into r3;
    cast r0.owner r3 into r4 as credits.record;
    async transfer_private_to_public r1 r2 into r5;
    output r4 as credits.record;
    output r5 as credits.aleo/transfer_private_to_public.future;

finalize transfer_private_to_public:
    input r0 as address.public;
    input r1 as u64.public;
    get.or_use account[r0] 0u64 into r2;
    add r1 r2 into r3;
    set r3 into account[r0];

function transfer_public_to_private:
    input r0 as address.private;
    input r1 as u64.public;
    cast r0 r1 into r2 as credits.record;
    async transfer_public_to_private self.caller r1 into r3;
    output r2 as credits.record;
    output r3 as credits.aleo/transfer_public_to_private.future;

finalize transfer_public_to_private:
    input r0 as address.public;
    input r1 as u64.public;
    get account[r0] into r2;
    sub r2 r1 into r3;
    set r3 into account[r0];

function join:
    input r0 as credits.record;
    input r1 as credits.record;
    add r0.microcredits r1.microcredits into r2;
    cast r0.owner r2 into r3 as credits.record;
    output r3 as credits.record;

function split:
    input r0 as credits.record;
    input r1 as u64.private;
    sub r0.microcredits r1 into r2;
    sub r2 10000u64 into r3;
    cast r0.owner r1 into r4 as credits.record;
    cast r0.owner r3 into r5 as credits.record;
    output r4 as credits.record;
    output r5 as credits.record;

function fee_private:
    input r0 as credits.record;
    input r1 as u64.public;
    input r2 as u64.public;
    input r3 as field.public;
    assert.neq r1 0u64 ;
    assert.neq r3 0field ;
    add r1 r2 into r4;
    sub r0.microcredits r4 into r5;
    cast r0.owner r5 into r6 as credits.record;
    output r6 as credits.record;

function fee_public:
    input r0 as u64.public;
    input r1 as u64.public;
    input r2 as field.public;
    assert.neq r0 0u64 ;
    assert.neq r2 0field ;
    add r0 r1 into r3;
    async fee_public self.caller r3 into r4;
    output r4 as credits.aleo/fee_public.future;

finalize fee_public:
    input r0 as address.public;
    input r1 as u64.public;
    get account[r0] into r2;
    sub r2 r1 into r3;
    set r3 into account[r0];
