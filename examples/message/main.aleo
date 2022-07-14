program message.aleo;
interface message:
    first as field;
    second as field;

function main:
    input r0 as message.private;
    cast r0.first r0.second into r1 as message;
    add r1.first r1.second into r2;
    output r2 as field.private;
