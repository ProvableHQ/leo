import dep.aleo;

program importer.aleo;

function call_id:
    input r0 as u32.private;
    call dep.aleo/id r0 into r1;
    output r1 as u32.private;
