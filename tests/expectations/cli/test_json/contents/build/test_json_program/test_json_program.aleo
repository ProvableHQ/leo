program test_json_program.aleo;

mapping expected_checksum:
    key as boolean.public;
    value as [u8; 32u32].public;

function main:
    input r0 as u32.public;
    input r1 as u32.private;
    add r0 r1 into r2;
    output r2 as u32.private;

constructor:
    branch.eq edition 0u16 to end;
    get expected_checksum[true] into r0;
    assert.eq checksum r0;
    position end;
