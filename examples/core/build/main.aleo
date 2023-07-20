program core.aleo;



function main:
    input r0 as field.private;
    hash.bhp256 r0 into r1 as field;    hash.psd2 r1 into r2 as field;    commit.bhp256 r2 1scalar into r3 as field;    output r3 as field.private;
