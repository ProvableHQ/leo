/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    // Test Snark::verify with correct types in an async transition
    async transition main(
        vk: [u8; 128],
        varuna_version: u8,
        inputs: [field; 4],
        proof: [u8; 256],
        // Batch types
        vks: [[u8; 128]; 2],
        batch_inputs: [[[field; 4]; 3]; 2],
    ) -> Future {
        return finalize(vk, varuna_version, inputs, proof, vks, batch_inputs);
    }

    async function finalize(
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
}
