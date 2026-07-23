program test.aleo {
    fn unused_deserialize(bits: [bool; 253]) -> u8 {
        let value: field = Deserialize::from_bits_raw::[field](bits);
        return 1u8;
    }

    fn unused_commit(value: field, randomness: scalar) -> u8 {
        let commitment: field = BHP256::commit_to_field(value, randomness);
        return 1u8;
    }

    fn unused_finalize_intrinsics(
        sig: [u8; 65],
        addr: [u8; 33],
        digest: [u8; 32],
        vk: [u8; 128],
        varuna_version: u8,
        inputs: [field; 4],
        proof: [u8; 256],
        vks: [[u8; 128]; 2],
        batch_inputs: [[[field; 4]; 3]; 2],
    ) -> Final {
        return final { finalize(sig, addr, digest, vk, varuna_version, inputs, proof, vks, batch_inputs); };
    }

    fn used_deserialize(bits: [bool; 253]) -> field {
        let value: field = Deserialize::from_bits_raw::[field](bits);
        return value;
    }

    @noupgrade
    constructor() {}
}

export final fn finalize(
    sig: [u8; 65],
    addr: [u8; 33],
    digest: [u8; 32],
    vk: [u8; 128],
    varuna_version: u8,
    inputs: [field; 4],
    proof: [u8; 256],
    vks: [[u8; 128]; 2],
    batch_inputs: [[[field; 4]; 3]; 2],
) {
    let ecdsa_result: bool = ECDSA::verify_digest(sig, addr, digest);
    let snark_result: bool = Snark::verify(vk, varuna_version, inputs, proof);
    let batch_result: bool = Snark::verify_batch(vks, varuna_version, batch_inputs, proof);
}
