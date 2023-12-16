program child.aleo;

record A:
    owner as address.private;
    val as u32.private;


function mint:
    input r0 as address.private;
    input r1 as u32.private;
    cast r0 r1 into r2 as A.record;
    output r2 as A.record;
