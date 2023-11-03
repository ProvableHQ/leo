import credits.aleo;
program super_simple.aleo;






function main:
    input r0 as credits.aleo/credits.record;
    input r1 as address.private;
    input r2 as u64.private;
    call credits.aleo/transfer_private r0 r1 r2 into r3 r4;
    output r3 as credits.aleo/credits.record;
    output r4 as credits.aleo/credits.record;


function generate:
    input r0 as address.private;
    input r1 as u64.private;
    call credits.aleo/transfer_public_to_private r0 r1 into r2;
    output r2 as credits.aleo/credits.record;
