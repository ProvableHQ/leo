import grandchild.aleo;
import child1.aleo;
import child2.aleo;
program parent.aleo;

function main:
    input r0 as u32.private;
    call child1.aleo/main r0 into r1;
    call child2.aleo/main r0 into r2;
    add r1 r2 into r3;
    output r3 as u32.private;

constructor:
    assert.eq edition 0u16;
