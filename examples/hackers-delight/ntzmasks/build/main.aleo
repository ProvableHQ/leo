program ntzmasks.aleo;



function main:
    input r0 as u32.public;
    is.eq r0 0u32 into r1;
    and r0 65535u32 into r2;
    is.eq r2 0u32 into r3;
    add 1u8 16u8 into r4;
    shr r0 16u8 into r5;
    ternary r3 r4 1u8 into r6;
    ternary r3 r5 r0 into r7;
    and r7 255u32 into r8;
    is.eq r8 0u32 into r9;
    add r6 8u8 into r10;
    shr r7 8u8 into r11;
    ternary r9 r10 r6 into r12;
    ternary r9 r11 r7 into r13;
    and r13 15u32 into r14;
    is.eq r14 0u32 into r15;
    add r12 4u8 into r16;
    shr r13 4u8 into r17;
    ternary r15 r16 r12 into r18;
    ternary r15 r17 r13 into r19;
    and r19 3u32 into r20;
    is.eq r20 0u32 into r21;
    add r18 2u8 into r22;
    shr r19 2u8 into r23;
    ternary r21 r22 r18 into r24;
    ternary r21 r23 r19 into r25;
    and r25 1u32 into r26;
    is.eq r26 1u32 into r27;
    sub r24 1u8 into r28;
    ternary r27 r28 r24 into r29;
    ternary r1 32u8 r29 into r30;
    output r30 as u8.private;
