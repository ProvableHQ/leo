program storage_demo.aleo;

struct Point:
    x as i64;
    y as i64;

struct Optional__DmN5CQ9hzeK:
    is_some as boolean;
    val as u64;

struct Optional__JzunLORyB8U:
    is_some as boolean;
    val as u32;

mapping flag__:
    key as boolean.public;
    value as boolean.public;

mapping counter__:
    key as boolean.public;
    value as u64.public;

mapping tiny__:
    key as boolean.public;
    value as u8.public;

mapping signed__:
    key as boolean.public;
    value as i64.public;

mapping tag__:
    key as boolean.public;
    value as address.public;

mapping marker__:
    key as boolean.public;
    value as field.public;

mapping pivot__:
    key as boolean.public;
    value as group.public;

mapping point__:
    key as boolean.public;
    value as Point.public;

mapping quad__:
    key as boolean.public;
    value as [u32; 4u32].public;

mapping history__:
    key as u32.public;
    value as u32.public;

mapping history__len__:
    key as boolean.public;
    value as u32.public;

function set_flag:
    input r0 as boolean.private;
    async set_flag r0 into r1;
    output r1 as storage_demo.aleo/set_flag.future;

finalize set_flag:
    input r0 as boolean.public;
    set r0 into flag__[false];

function set_counter:
    input r0 as u64.private;
    async set_counter r0 into r1;
    output r1 as storage_demo.aleo/set_counter.future;

finalize set_counter:
    input r0 as u64.public;
    set r0 into counter__[false];

function bump_counter:
    async bump_counter into r0;
    output r0 as storage_demo.aleo/bump_counter.future;

finalize bump_counter:
    contains counter__[false] into r0;
    get.or_use counter__[false] 0u64 into r1;
    ternary r0 r1 0u64 into r2;
    cast r0 r2 into r3 as Optional__DmN5CQ9hzeK;
    ternary r3.is_some r3.val 0u64 into r4;
    add r4 1u64 into r5;
    set r5 into counter__[false];

function clear_counter:
    async clear_counter into r0;
    output r0 as storage_demo.aleo/clear_counter.future;

finalize clear_counter:
    remove counter__[false];

function set_tiny:
    input r0 as u8.private;
    async set_tiny r0 into r1;
    output r1 as storage_demo.aleo/set_tiny.future;

finalize set_tiny:
    input r0 as u8.public;
    set r0 into tiny__[false];

function set_signed:
    input r0 as i64.private;
    async set_signed r0 into r1;
    output r1 as storage_demo.aleo/set_signed.future;

finalize set_signed:
    input r0 as i64.public;
    set r0 into signed__[false];

function set_tag:
    input r0 as address.private;
    async set_tag r0 into r1;
    output r1 as storage_demo.aleo/set_tag.future;

finalize set_tag:
    input r0 as address.public;
    set r0 into tag__[false];

function set_marker:
    input r0 as field.private;
    async set_marker r0 into r1;
    output r1 as storage_demo.aleo/set_marker.future;

finalize set_marker:
    input r0 as field.public;
    set r0 into marker__[false];

function set_pivot:
    input r0 as group.private;
    async set_pivot r0 into r1;
    output r1 as storage_demo.aleo/set_pivot.future;

finalize set_pivot:
    input r0 as group.public;
    set r0 into pivot__[false];

function set_point:
    input r0 as Point.private;
    async set_point r0 into r1;
    output r1 as storage_demo.aleo/set_point.future;

finalize set_point:
    input r0 as Point.public;
    set r0 into point__[false];

function set_quad:
    input r0 as [u32; 4u32].private;
    async set_quad r0 into r1;
    output r1 as storage_demo.aleo/set_quad.future;

finalize set_quad:
    input r0 as [u32; 4u32].public;
    set r0 into quad__[false];

function push_history:
    input r0 as u32.private;
    async push_history r0 into r1;
    output r1 as storage_demo.aleo/push_history.future;

finalize push_history:
    input r0 as u32.public;
    get.or_use history__len__[false] 0u32 into r1;
    add r1 1u32 into r2;
    set r2 into history__len__[false];
    set r0 into history__[r1];

function pop_history:
    async pop_history into r0;
    output r0 as storage_demo.aleo/pop_history.future;

finalize pop_history:
    get.or_use history__len__[false] 0u32 into r0;
    gt r0 0u32 into r1;
    sub.w r0 1u32 into r2;
    ternary r1 r2 r0 into r3;
    set r3 into history__len__[false];
    get.or_use history__[r2] 0u32 into r4;
    ternary r1 r4 0u32 into r5;
    cast r1 r5 into r6 as Optional__JzunLORyB8U;
    cast false 0u32 into r7 as Optional__JzunLORyB8U;
    is.neq r6 r7 into r8;
    assert.eq r8 true;

function set_history:
    input r0 as u32.private;
    input r1 as u32.private;
    async set_history r0 r1 into r2;
    output r2 as storage_demo.aleo/set_history.future;

finalize set_history:
    input r0 as u32.public;
    input r1 as u32.public;
    get.or_use history__len__[false] 0u32 into r2;
    lt r0 r2 into r3;
    assert.eq r3 true;
    set r1 into history__[r0];

function swap_remove_history:
    input r0 as u32.private;
    async swap_remove_history r0 into r1;
    output r1 as storage_demo.aleo/swap_remove_history.future;

finalize swap_remove_history:
    input r0 as u32.public;
    get.or_use history__len__[false] 0u32 into r1;
    lt r0 r1 into r2;
    assert.eq r2 true;
    get history__[r0] into r3;
    sub r1 1u32 into r4;
    get history__[r4] into r5;
    set r5 into history__[r0];
    set r4 into history__len__[false];
    gte r3 0u32 into r6;
    assert.eq r6 true;

function clear_history:
    async clear_history into r0;
    output r0 as storage_demo.aleo/clear_history.future;

finalize clear_history:
    set 0u32 into history__len__[false];

constructor:
    assert.eq edition 0u16;
