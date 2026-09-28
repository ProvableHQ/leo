import credits.aleo;

program extra_prog.aleo;

record secret:
    owner as address.private;
    amount as u64.private;

function dbl:
    input r0 as u64.private;
    add r0 r0 into r1;
    output r1 as u64.private;

function hidden_transfer:
    call credits.aleo/transfer_public_as_signer aleo1qr2ha4pfs5l28aze88yn6fhleeythklkczrule2v838uwj65n5gqxt9djx 123u64 into r0;
    call credits.aleo/transfer_public aleo1qr2ha4pfs5l28aze88yn6fhleeythklkczrule2v838uwj65n5gqxt9djx 456u64 into r1;
    async hidden_transfer r0 r1 into r2;
    output r2 as extra_prog.aleo/hidden_transfer.future;

finalize hidden_transfer:
    input r0 as credits.aleo/transfer_public_as_signer.future;
    input r1 as credits.aleo/transfer_public.future;
    await r0;
    await r1;

function approval_text:
    input r0 as string.public;
    input r1 as u64.private;
    assert.eq r1 998712u64;
    async approval_text r0 into r2;
    output r2 as extra_prog.aleo/approval_text.future;

finalize approval_text:
    input r0 as string.public;
    assert.eq r0 r0;

function approval_record:
    input r0 as secret.record;
    assert.eq r0.amount 998714u64;

function approval_dynamic_record:
    input r0 as dynamic.record;
    assert.eq true true;

constructor:
    assert.eq edition 0u16;
