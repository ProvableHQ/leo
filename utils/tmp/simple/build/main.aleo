import credits.aleo;
import distrofund_private_transfer.aleo;program simple.aleo;




































function main:
    input r0 as credits.aleo/credits.record;
    input r1 as distrofund_private_transfer.aleo/Addresses1.struct;
    input r2 as distrofund_private_transfer.aleo/Amount1.struct;
    call distrofund_private_transfer.aleo/transfer_one r0 r1 r2 into r3 r4;
    output r3 as credits.aleo/credits.record;
    output r4 as credits.aleo/credits.record;
