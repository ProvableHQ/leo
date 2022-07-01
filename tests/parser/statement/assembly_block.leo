/*
namespace: ParseStatement
expectation: Pass
*/

assembly{}

assembly {}

assembly {
    add r1 r2 into r3;
}


assembly {
    add r1 r2 into r3;
    eq r3 0u8 into r4;
    ter r4 1u8 2u8 into r5;
}