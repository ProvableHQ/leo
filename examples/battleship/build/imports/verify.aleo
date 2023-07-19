program verify.aleo;



closure bitcount:
    input r0 as u64;
    div r0 2u64 into r1;
    div r0 4u64 into r2;
    div r0 8u64 into r3;
    and r1 8608480567731124087u64 into r4;
    and r2 3689348814741910323u64 into r5;
    and r3 1229782938247303441u64 into r6;
    sub r0 r4 into r7;
    sub r7 r5 into r8;
    sub r8 r6 into r9;
    div r9 16u64 into r10;
    add r9 r10 into r11;
    and r11 1085102592571150095u64 into r12;
    rem r12 255u64 into r13;
    output r13 as u64;


closure adjacency_check:
    input r0 as u64;
    input r1 as u64;
    div r0 r1 into r2;
    is.eq r2 0u64 into r3;
    ternary r3 3u64 r2 into r4;
    sub r4 1u64 into r5;
    and r5 r4 into r6;
    is.eq r6 0u64 into r7;
    output r7 as boolean;


closure horizontal_check:
    input r0 as u64;
    input r1 as u64;
    rem r0 255u64 into r2;
    div r2 r1 into r3;
    is.eq r3 0u64 into r4;
    ternary r4 3u64 r3 into r5;
    sub r5 1u64 into r6;
    and r6 r5 into r7;
    is.eq r7 0u64 into r8;
    output r8 as boolean;


function validate_ship:
    input r0 as u64.private;
    input r1 as u64.private;
    input r2 as u64.private;
    input r3 as u64.private;
    call bitcount r0 into r4;
    assert.eq r4 r1;
    call adjacency_check r0 r2 into r5;
    call horizontal_check r0 r2 into r6;
    and r5 r6 into r7;
    call adjacency_check r0 r3 into r8;
    or r7 r8 into r9;
    output r9 as boolean.private;


function create_board:
    input r0 as u64.private;
    input r1 as u64.private;
    input r2 as u64.private;
    input r3 as u64.private;
    or r0 r1 into r4;
    or r4 r2 into r5;
    or r5 r3 into r6;
    call bitcount r6 into r7;
    assert.eq r7 14u64;
    output r6 as u64.private;
