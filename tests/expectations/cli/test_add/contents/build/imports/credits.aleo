program credits.aleo;

mapping committee:
    key as address.public;
    value as committee_state.public;

struct committee_state:
    is_open as boolean;
    commission as u8;

mapping delegated:
    key as address.public;
    value as u64.public;

mapping metadata:
    key as address.public;
    value as u32.public;

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

mapping withdraw:
    key as address.public;
    value as address.public;

mapping pool:
    key as address.public;
    value as u64.public;

record credits:
    owner as address.private;
    microcredits as u64.private;

function bond_validator:
    input r0 as address.public;
    input r1 as u64.public;
    input r2 as u8.public;
    assert.neq self.signer r0;
    gte r1 1_000_000u64 into r3;
    assert.eq r3 true;
    gt r2 100u8 into r4;
    assert.neq r4 true;
    async bond_validator self.signer r0 r1 r2 into r5;
    output r5 as credits.aleo/bond_validator.future;

finalize bond_validator:
    input r0 as address.public;
    input r1 as address.public;
    input r2 as u64.public;
    input r3 as u8.public;
    get.or_use withdraw[r0] r1 into r4;
    assert.eq r1 r4;
    cast true r3 into r5 as committee_state;
    get.or_use committee[r0] r5 into r6;
    assert.eq r3 r6.commission;
    cast r0 0u64 into r7 as bond_state;
    get.or_use bonded[r0] r7 into r8;
    assert.eq r8.validator r0;
    add r8.microcredits r2 into r9;
    cast r0 r9 into r10 as bond_state;
    get.or_use delegated[r0] 0u64 into r11;
    add r2 r11 into r12;
    gte r12 10_000_000_000_000u64 into r13;
    assert.eq r13 true;
    get account[r0] into r14;
    sub r14 r2 into r15;
    contains committee[r0] into r16;
    branch.eq r16 true to validator_in_committee;
        set r4 into withdraw[r0];
        gte r2 100_000_000u64 into r17;
        assert.eq r17 true;
        get.or_use metadata[aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc] 0u32 into r18;
        add r18 1u32 into r19;
        set r19 into metadata[aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc];
        contains unbonding[r0] into r20;
        assert.eq r20 false;
    position validator_in_committee;
    set r6 into committee[r0];
    set r12 into delegated[r0];
    set r10 into bonded[r0];
    set r15 into account[r0];

function bond_public:
    input r0 as address.public;
    input r1 as address.public;
    input r2 as u64.public;
    gte r2 1_000_000u64 into r3;
    assert.eq r3 true;
    assert.neq self.caller r0;
    async bond_public self.caller r0 r1 r2 into r4;
    output r4 as credits.aleo/bond_public.future;

finalize bond_public:
    input r0 as address.public;
    input r1 as address.public;
    input r2 as address.public;
    input r3 as u64.public;
    get.or_use withdraw[r0] r2 into r4;
    assert.eq r2 r4;
    contains bonded[r0] into r5;
    branch.eq r5 true to continue_bond_delegator;
        set r2 into withdraw[r0];
        cast true 0u8 into r6 as committee_state;
        get.or_use committee[r1] r6 into r7;
        assert.eq r7.is_open true;
        get.or_use metadata[aleo1qgqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqanmpl0] 0u32 into r8;
        add r8 1u32 into r9;
        lte r9 100_000u32 into r10;
        assert.eq r10 true;
        set r9 into metadata[aleo1qgqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqanmpl0];
    position continue_bond_delegator;
    cast r1 0u64 into r11 as bond_state;
    get.or_use bonded[r0] r11 into r12;
    assert.eq r12.validator r1;
    add r12.microcredits r3 into r13;
    gte r13 10_000_000_000u64 into r14;
    assert.eq r14 true;
    cast r1 r13 into r15 as bond_state;
    get account[r0] into r16;
    sub r16 r3 into r17;
    get.or_use delegated[r1] 0u64 into r18;
    add r3 r18 into r19;
    contains unbonding[r1] into r20;
    assert.eq r20 false;
    set r15 into bonded[r0];
    set r17 into account[r0];
    set r19 into delegated[r1];

function unbond_public:
    input r0 as address.public;
    input r1 as u64.public;
    async unbond_public self.caller r0 r1 into r2;
    output r2 as credits.aleo/unbond_public.future;

finalize unbond_public:
    input r0 as address.public;
    input r1 as address.public;
    input r2 as u64.public;
    add block.height 360u32 into r3;
    cast 0u64 r3 into r4 as unbond_state;
    get bonded[r1] into r5;
    get withdraw[r1] into r6;
    is.eq r0 r6 into r7;
    contains withdraw[r5.validator] into r8;
    get.or_use withdraw[r5.validator] aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc into r9;
    is.eq r0 r9 into r10;
    and r8 r10 into r11;
    or r7 r11 into r12;
    assert.eq r12 true;
    is.eq r5.validator r1 into r13;
    branch.eq r13 true to unbond_validator;
        get.or_use unbonding[r1] r4 into r14;
        get delegated[r5.validator] into r15;
        sub r5.microcredits r2 into r16;
        lt r16 10_000_000_000u64 into r17;
        or r11 r17 into r18;
        ternary r18 r5.microcredits r2 into r19;
        add r14.microcredits r19 into r20;
        cast r20 r3 into r21 as unbond_state;
        set r21 into unbonding[r1];
        sub r15 r19 into r22;
        set r22 into delegated[r5.validator];
        branch.eq r18 true to remove_delegator;
            cast r5.validator r16 into r23 as bond_state;
            set r23 into bonded[r1];
            branch.eq true true to end_unbond_delegator;
        position remove_delegator;
            remove bonded[r1];
            get metadata[aleo1qgqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqanmpl0] into r24;
            sub r24 1u32 into r25;
            set r25 into metadata[aleo1qgqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqanmpl0];
        position end_unbond_delegator;
            gte r22 10_000_000_000_000u64 into r26;
            branch.eq r26 true to end;
    position unbond_validator;
        contains committee[r5.validator] into r27;
        nor r13 r27 into r28;
        branch.eq r28 true to end;
        get committee[r5.validator] into r29;
        get bonded[r5.validator] into r30;
        get delegated[r5.validator] into r31;
        lt r31 10_000_000_000_000u64 into r32;
        branch.eq r32 true to remove_validator;
        sub r31 r2 into r33;
        sub r30.microcredits r2 into r34;
        gte r34 100_000_000u64 into r35;
        gte r33 10_000_000_000_000u64 into r36;
        and r35 r36 into r37;
        branch.eq r37 false to remove_validator;
        get.or_use unbonding[r5.validator] r4 into r38;
        add r38.microcredits r2 into r39;
        cast r39 r3 into r40 as unbond_state;
        set r40 into unbonding[r5.validator];
        set r33 into delegated[r5.validator];
        cast r5.validator r34 into r41 as bond_state;
        set r41 into bonded[r5.validator];
        branch.eq true true to end;
    position remove_validator;
        remove committee[r5.validator];
        get metadata[aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc] into r42;
        sub r42 1u32 into r43;
        set r43 into metadata[aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc];
        sub r31 r30.microcredits into r44;
        set r44 into delegated[r5.validator];
        remove bonded[r5.validator];
        get.or_use unbonding[r5.validator] r4 into r45;
        add r30.microcredits r45.microcredits into r46;
        cast r46 r3 into r47 as unbond_state;
        set r47 into unbonding[r5.validator];
    position end;

function claim_unbond_public:
    input r0 as address.public;
    async claim_unbond_public r0 into r1;
    output r1 as credits.aleo/claim_unbond_public.future;

finalize claim_unbond_public:
    input r0 as address.public;
    get unbonding[r0] into r1;
    gte block.height r1.height into r2;
    assert.eq r2 true;
    get withdraw[r0] into r3;
    get.or_use account[r3] 0u64 into r4;
    add r1.microcredits r4 into r5;
    set r5 into account[r3];
    remove unbonding[r0];
    contains bonded[r0] into r6;
    branch.eq r6 true to end;
    remove withdraw[r0];
    position end;

function set_validator_state:
    input r0 as boolean.public;
    async set_validator_state self.caller r0 into r1;
    output r1 as credits.aleo/set_validator_state.future;

finalize set_validator_state:
    input r0 as address.public;
    input r1 as boolean.public;
    get committee[r0] into r2;
    cast r1 r2.commission into r3 as committee_state;
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

function transfer_public_as_signer:
    input r0 as address.public;
    input r1 as u64.public;
    async transfer_public_as_signer self.signer r0 r1 into r2;
    output r2 as credits.aleo/transfer_public_as_signer.future;

finalize transfer_public_as_signer:
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
    sub r2 10_000u64 into r3;
    cast r0.owner r1 into r4 as credits.record;
    cast r0.owner r3 into r5 as credits.record;
    output r4 as credits.record;
    output r5 as credits.record;

function fee_private:
    input r0 as credits.record;
    input r1 as u64.public;
    input r2 as u64.public;
    input r3 as field.public;
    assert.neq r1 0u64;
    assert.neq r3 0field;
    add r1 r2 into r4;
    sub r0.microcredits r4 into r5;
    cast r0.owner r5 into r6 as credits.record;
    output r6 as credits.record;

function fee_public:
    input r0 as u64.public;
    input r1 as u64.public;
    input r2 as field.public;
    assert.neq r0 0u64;
    assert.neq r2 0field;
    add r0 r1 into r3;
    async fee_public self.signer r3 into r4;
    output r4 as credits.aleo/fee_public.future;

finalize fee_public:
    input r0 as address.public;
    input r1 as u64.public;
    get account[r0] into r2;
    sub r2 r1 into r3;
    set r3 into account[r0];

function upgrade:
    input r0 as credits.record;
    lte r0.microcredits 1_000_000_000_000u64 into r1;
    assert.eq r1 true;
    cast r0.owner r0.microcredits into r2 as credits.record;
    async upgrade r0.microcredits into r3;
    output r2 as credits.record;
    output r3 as credits.aleo/upgrade.future;

finalize upgrade:
    input r0 as u64.public;
    get.or_use pool[aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc] 0u64 into r1;
    add r1 r0 into r2;
    lte r2 4_000_000_000_000u64 into r3;
    assert.eq r3 true;
    set r2 into pool[aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc];