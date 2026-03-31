program persistent_counter.aleo;

mapping counts:
    key as address.public;
    value as u64.public;

function increment:
    async increment self.caller into r0;
    output r0 as persistent_counter.aleo/increment.future;

finalize increment:
    input r0 as address.public;
    get.or_use counts[r0] 0u64 into r1;
    add r1 1u64 into r2;
    set r2 into counts[r0];

constructor:
    assert.eq edition 0u16;
