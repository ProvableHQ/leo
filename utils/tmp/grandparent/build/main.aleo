import child.aleo;
import parent.aleo;
program grandparent.aleo;



function double_wrapper_mint:
    input r0 as address.private;
    input r1 as u32.private;
    call parent.aleo/wrapper_mint r0 r1 into r2;
    output r2 as child.aleo/A.record;
