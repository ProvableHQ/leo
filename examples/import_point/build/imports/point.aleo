program point.aleo;

interface point:
    x as u32;
    y as u32;

function new:
    input r0 as u32.private;
    input r1 as u32.private;
    cast r0 r1 into r2 as point;
    output r2 as point.private;

function sum:
    input r0 as point.private;
    input r1 as point.private;
    add r0.x r1.x into r2;
    add r0.y r1.y into r3;
    cast r2 r3 into r4 as point;
    output r4 as point.private;
