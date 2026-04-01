program test_dynamic_call_with_flag.aleo;

function main:
    input r0 as identifier.private;
    input r1 as identifier.private;
    input r2 as identifier.private;
    input r3 as u64.private;
    cast r0 into r4 as field;
    cast r1 into r5 as field;
    cast r2 into r6 as field;
    call.dynamic r4 r5 r6 with r3 (as u64.private) into r7 (as u64.private);
    output r7 as u64.private;

constructor:
    assert.eq edition 0u16;
