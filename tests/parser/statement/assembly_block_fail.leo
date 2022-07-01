/*
namespace: ParseStatement
expectation: Fail
*/

assembly[}

assembly{]

assembly[]

assembli {}

assembly {
    add r1 r2 into r3
}

assembly {
    thisopcodedoesnotexist r1 r2 into r3;
}

assembly {
    add r1 r2 thishouldbeinto r3;
}

assembly {
    add r1 (1u8 + 2u8) into r3;
}

