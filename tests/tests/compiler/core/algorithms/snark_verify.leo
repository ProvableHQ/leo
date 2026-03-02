/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    // Test Snark::verify with correct types in an async transition
    fn main(
        vk: [u8; 128],
        varuna_version: u8,
        inputs: [field; 4],
        proof: [u8; 256],
        // Batch types
        vks: [[u8; 128]; 2],
        batch_inputs: [[[field; 4]; 3]; 2],
    ) -> Final {
        return final { finalize(vk, varuna_version, inputs, proof, vks, batch_inputs); };
    }

    // Boundary test: exactly 32 circuits and exactly 128 instances (32 * 4 = 128) are accepted.
    fn boundary(
        vks32: [[u8; 128]; 32],
        batch_inputs32: [[[field; 4]; 4]; 32],
        proof: [u8; 256],
        v: u8,
    ) -> Final {
        return final { finalize_boundary(vks32, batch_inputs32, proof, v); };
    }
}

final fn finalize(
    vk: [u8; 128],
    varuna_version: u8,
    inputs: [field; 4],
    proof: [u8; 256],
    vks: [[u8; 128]; 2],
    batch_inputs: [[[field; 4]; 3]; 2],
) {
    // Test Snark::verify (single)
    let v1: bool = Snark::verify(vk, varuna_version, inputs, proof);

    // Test Snark::verify_batch
    let v2: bool = Snark::verify_batch(vks, varuna_version, batch_inputs, proof);

    // Prevent DCE
    assert_eq(v1, v1);
    assert_eq(v2, v2);
}

final fn finalize_boundary(
    vks32: [[u8; 128]; 32],
    batch_inputs32: [[[field; 4]; 4]; 32],
    proof: [u8; 256],
    v: u8,
) {
    let r: bool = Snark::verify_batch(vks32, v, batch_inputs32, proof);
    assert_eq(r, r);
}
