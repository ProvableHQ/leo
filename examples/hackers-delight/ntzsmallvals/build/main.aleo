program ntzsmallvals.aleo;



function main:
    input r0 as u32.public;
    is.eq r0 0u32 into r1;
    shl.w r0 16u8 into r2;
    is.neq r2 0u32 into r3;
    sub 31u8 16u8 into r4;
    ternary r3 r4 31u8 into r5;
    ternary r3 r2 r0 into r6;
    shl.w r6 8u8 into r7;
    is.neq r7 0u32 into r8;
    sub r5 8u8 into r9;
    ternary r8 r9 r5 into r10;
    ternary r8 r7 r6 into r11;
    shl.w r11 4u8 into r12;
    is.neq r12 0u32 into r13;
    sub r10 4u8 into r14;
    ternary r13 r14 r10 into r15;
    ternary r13 r12 r11 into r16;
    shl.w r16 2u8 into r17;
    is.neq r17 0u32 into r18;
    sub r15 2u8 into r19;
    ternary r18 r19 r15 into r20;
    ternary r18 r17 r16 into r21;
    shl.w r21 1u8 into r22;
    is.neq r22 0u32 into r23;
    sub r20 1u8 into r24;
    ternary r23 r24 r20 into r25;
    ternary r1 32u8 r25 into r26;
    output r26 as u8.private;
