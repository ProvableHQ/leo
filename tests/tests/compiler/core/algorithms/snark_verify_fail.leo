/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    async transition main(
        bad_vk: u32,
        bad_varuna: u32,
        bad_inputs: [u8; 4],
        bad_proof: u64,
        vk: [u8; 128],
        varuna_version: u8,
        inputs: [field; 4],
        proof: [u8; 256],
        // Batch types
        batch_vks_flat: [u8; 128],
        batch_vks: [[u8; 128]; 3],
        batch_inputs_2d: [[field; 4]; 3],
        batch_inputs_mismatch: [[[field; 4]; 3]; 2],
        // Nested array types (2D where 1D expected)
        nested_vk: [[u8; 3]; 4],
        nested_inputs: [[field; 2]; 3],
        nested_proof: [[u8; 5]; 2],
    ) -> Future {
        return finalize(
            bad_vk, bad_varuna, bad_inputs, bad_proof,
            vk, varuna_version, inputs, proof,
            batch_vks_flat, batch_vks, batch_inputs_2d, batch_inputs_mismatch,
            nested_vk, nested_inputs, nested_proof,
        );
    }

    async function finalize(
        bad_vk: u32,
        bad_varuna: u32,
        bad_inputs: [u8; 4],
        bad_proof: u64,
        vk: [u8; 128],
        varuna_version: u8,
        inputs: [field; 4],
        proof: [u8; 256],
        batch_vks_flat: [u8; 128],
        batch_vks: [[u8; 128]; 3],
        batch_inputs_2d: [[field; 4]; 3],
        batch_inputs_mismatch: [[[field; 4]; 3]; 2],
        nested_vk: [[u8; 3]; 4],
        nested_inputs: [[field; 2]; 3],
        nested_proof: [[u8; 5]; 2],
    ) {
        // 1. Fail: vk is u32, not [u8; N]
        let v1: bool = Snark::verify(bad_vk, varuna_version, inputs, proof);

        // 2. Fail: varuna_version is u32, not u8
        let v2: bool = Snark::verify(vk, bad_varuna, inputs, proof);

        // 3. Fail: inputs has wrong element type [u8; 4] not [field; N]
        let v3: bool = Snark::verify(vk, varuna_version, bad_inputs, proof);

        // 4. Fail: proof is u64, not [u8; N]
        let v4: bool = Snark::verify(vk, varuna_version, inputs, bad_proof);

        // 5. Fail: batch vks is 1D [u8; 128] not 2D [[u8; N]; M]
        let v5: bool = Snark::verify_batch(batch_vks_flat, varuna_version, batch_inputs_mismatch, proof);

        // 6. Fail: batch inputs is 2D [[field; 4]; 3] not 3D [[[field; N]; P]; K]
        let v6: bool = Snark::verify_batch(batch_vks, varuna_version, batch_inputs_2d, proof);

        // 7. Fail: batch dimension mismatch — 3 vks but 2 circuits in inputs
        let v7: bool = Snark::verify_batch(batch_vks, varuna_version, batch_inputs_mismatch, proof);

        // 8. Fail: vk is [[u8; 3]; 4] — 2D array, Snark::verify expects [u8; N]
        let v8: bool = Snark::verify(nested_vk, varuna_version, inputs, proof);

        // 9. Fail: inputs is [[field; 2]; 3] — 2D array, Snark::verify expects [field; N]
        let v9: bool = Snark::verify(vk, varuna_version, nested_inputs, proof);

        // 10. Fail: proof is [[u8; 5]; 2] — 2D array, Snark::verify expects [u8; N]
        let v10: bool = Snark::verify(vk, varuna_version, inputs, nested_proof);

        assert_eq(v1, v1);
        assert_eq(v2, v2);
        assert_eq(v3, v3);
        assert_eq(v4, v4);
        assert_eq(v5, v5);
        assert_eq(v6, v6);
        assert_eq(v7, v7);
        assert_eq(v8, v8);
        assert_eq(v9, v9);
        assert_eq(v10, v10);
    }
}
