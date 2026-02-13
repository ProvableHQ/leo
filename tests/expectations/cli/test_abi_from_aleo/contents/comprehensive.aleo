program comprehensive.aleo;

record Token:
    owner as address.private;
    amount as u64.public;
    data as field.private;

struct Point:
    x as i32;
    y as i32;

struct AllPrimitives:
    f_address as address;
    f_bool as boolean;
    f_field as field;
    f_group as group;
    f_scalar as scalar;
    f_i8 as i8;
    f_i16 as i16;
    f_i32 as i32;
    f_i64 as i64;
    f_i128 as i128;
    f_u8 as u8;
    f_u16 as u16;
    f_u32 as u32;
    f_u64 as u64;
    f_u128 as u128;

struct Rectangle:
    top_left as Point;
    bottom_right as Point;

mapping balances:
    key as address.public;
    value as u64.public;

mapping positions:
    key as address.public;
    value as Point.public;

mapping arrays:
    key as u32.public;
    value as [field; 2u32].public;

function single_primitive:
    input r0 as u32.private;
    add r0 1u32 into r1;
    output r1 as u32.private;

function multi_input:
    input r0 as u32.private;
    input r1 as u32.private;
    input r2 as u32.private;
    add r0 r1 into r3;
    add r3 r2 into r4;
    output r4 as u32.private;

function multi_output:
    input r0 as u32.private;
    add r0 1u32 into r1;
    output r0 as u32.private;
    output r1 as u32.private;

function visibility_modes:
    input r0 as u32.public;
    input r1 as u32.private;
    input r2 as u32.private;
    add r0 r1 into r3;
    add r3 r2 into r4;
    output r4 as u32.public;

function struct_io:
    input r0 as Point.private;
    add r0.x 1i32 into r1;
    add r0.y 1i32 into r2;
    cast r1 r2 into r3 as Point;
    output r3 as Point.private;

function nested_struct_io:
    input r0 as Rectangle.private;
    output r0 as Rectangle.private;

function array_io:
    input r0 as [u32; 4u32].private;
    output r0 as [u32; 4u32].private;

function nested_array_io:
    input r0 as [[u32; 2u32]; 3u32].private;
    output r0 as [[u32; 2u32]; 3u32].private;

function record_input:
    input r0 as Token.record;
    output r0.amount as u64.private;

function record_output:
    input r0 as address.private;
    input r1 as u64.private;
    input r2 as field.private;
    cast r0 r1 r2 into r3 as Token.record;
    output r3 as Token.record;

function all_primitives_io:
    input r0 as AllPrimitives.private;
    output r0 as AllPrimitives.private;

function async_with_finalize:
    input r0 as u64.public;
    async async_with_finalize r0 into r1;
    output r1 as comprehensive.aleo/async_with_finalize.future;

finalize async_with_finalize:
    input r0 as u64.public;
    add r0 1u64 into r1;

function update_balance:
    input r0 as address.public;
    input r1 as u64.public;
    async update_balance r0 r1 into r2;
    output r2 as comprehensive.aleo/update_balance.future;

finalize update_balance:
    input r0 as address.public;
    input r1 as u64.public;
    set r1 into balances[r0];

constructor:
    assert.eq edition 0u16;
