program bar.aleo;

function fetch_value:
    output 0u64 as u64.private;

function store_value:
    input r0 as u64.private;
    assert.eq r0 r0;

constructor:
    assert.eq edition 0u16;
