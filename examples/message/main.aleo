program message.aleo;

interface message:
    first as field;
    second as field;

function main:
    input r0 as message.private;
    add r0.first r0.second into r1;
    output r1 as field.private;
