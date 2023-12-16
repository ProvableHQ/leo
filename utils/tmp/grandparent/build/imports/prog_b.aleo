import prog_a.aleo;
program prog_b.aleo;



function wrapper_mint:
    input r0 as address.private;
    input r1 as u32.private;
    call prog_a.aleo/mint aleo1q30lfyggefvzzxqaaclzrn3wd94q4u8zzy8jhhfrcqrf306ayvqsdvj7s4 1u32 into r2;
    output r2 as prog_a.aleo/A.record;
