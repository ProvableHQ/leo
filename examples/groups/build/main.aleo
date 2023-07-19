program groups.aleo;



function main:
    input r0 as group.private;
    double r0 into r1;
    neg r1 into r2;
    mul r0 2scalar into r3;
    add r3 r2 into r4;
    add r4 group::GEN into r5;
    output r5 as group.private;
