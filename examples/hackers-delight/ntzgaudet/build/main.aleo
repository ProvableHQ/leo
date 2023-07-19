program ntzgaudet.aleo;



function main:
    input r0 as u32.public;
    sub.w 0u32 r0 into r1;
    and r0 r1 into r2;
    is.neq r2 0u32 into r3;
    ternary r3 0u8 1u8 into r4;
    and r2 65535u32 into r5;
    is.neq r5 0u32 into r6;
    ternary r6 0u8 16u8 into r7;
    and r2 16711935u32 into r8;
    is.neq r8 0u32 into r9;
    ternary r9 0u8 8u8 into r10;
    and r2 252645135u32 into r11;
    is.neq r11 0u32 into r12;
    ternary r12 0u8 4u8 into r13;
    and r2 858993459u32 into r14;
    is.neq r14 0u32 into r15;
    ternary r15 0u8 2u8 into r16;
    and r2 1431655765u32 into r17;
    is.neq r17 0u32 into r18;
    ternary r18 0u8 1u8 into r19;
    add r4 r7 into r20;
    add r20 r10 into r21;
    add r21 r13 into r22;
    add r22 r16 into r23;
    add r23 r19 into r24;
    output r24 as u8.private;
