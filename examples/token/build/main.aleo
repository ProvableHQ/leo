program token.aleo

record token:
    owner as address.private;
    gates as u64.private;
    amount as u64.private;

mapping account:
	key left as address.public;
	value right as u64.public;

function mint_public:
    input r0 as address.public;
    input r1 as u64.public;
    finalize r0 r1;

finalize mint_public:
    input r0 as address.public;
    input r1 as u64.public;
    increment account[r0] by r1;

function mint_private:
    input r0 as address.private;
    input r1 as u64.private;
    cast r0 0u64 r1 into r2 as token.record;
    output r2 as token.record;

function transfer_public:
    input r0 as address.public;
    input r1 as u64.public;
    finalize self.caller r0 r1;

finalize transfer_public:
    input r0 as address.public;
    input r1 as address.public;
    input r2 as u64.public;
    decrement account[r0] by r2;
    increment account[r1] by r2;

function transfer_private:
    input r0 as token.record;
    input r1 as address.private;
    input r2 as u64.private;
    sub r0.amount r2 into r3;
    cast r0.owner r0.gates r3 into r4 as token.record;
    cast r1 0u64 r2 into r5 as token.record;
    output r4 as token.record;
    output r5 as token.record;

function transfer_private_to_public:
    input r0 as token.record;
    input r1 as address.public;
    input r2 as u64.public;
    sub r0.amount r2 into r3;
    cast r0.owner r0.gates r3 into r4 as token.record;
    output r4 as token.record;
    finalize r1 r2;

finalize transfer_private_to_public:
    input r0 as address.public;
    input r1 as u64.public;
    increment account[r0] by r1;

function transfer_public_to_private:
    input r0 as address.public;
    input r1 as u64.public;
    cast r0 0u64 r1 into r2 as token.record;
    output r2 as token.record;
    finalize self.caller r1;

finalize transfer_public_to_private:
    input r0 as address.public;
    input r1 as u64.public;
    decrement account[r0] by r1;

