program message.aleo;

struct Message:
    first as field;
    second as field;


function main:
    input r0 as Message.private;
    cast r0.first r0.second into r1 as Message;
    add r1.first r1.second into r2;
    output r2 as field.private;
