program child.aleo;

struct Test:
    a as u32;
    b as u32;


function foo:
    input r0 as Test.private;
    output r0.a as u32.private;
