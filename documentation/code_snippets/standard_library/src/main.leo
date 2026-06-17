program stdlib_demo.aleo {

    fn hash_demos() {
        {
            // ANCHOR: std_hash_bhp256
            let h1: field = std::hash::bhp256::hash_u64_to_field(42u64);
            let h2: address = std::hash::bhp256::hash_field_to_address(1field);
            let h3: u32 = std::hash::bhp256::hash_bool_to_u32_raw(true);
            // ANCHOR_END: std_hash_bhp256
        }
        {
            // ANCHOR: std_hash_poseidon
            // Poseidon is the cheapest hash to prove in-circuit.
            let h: field = std::hash::poseidon2::hash_field_to_field(1field);
            // ANCHOR_END: std_hash_poseidon
        }
        {
            // ANCHOR: std_hash_pedersen
            // Pedersen-64 only accepts inputs up to 64 bits.
            let h: field = std::hash::pedersen64::hash_u32_to_field(7u32);
            // ANCHOR_END: std_hash_pedersen
        }
        {
            // ANCHOR: std_hash_keccak
            // Keccak/SHA-3 raw and to_bits variants require byte-aligned input.
            let h: field = std::hash::keccak256::hash_u64_to_field(0u64);
            let digest: [bool; 256] = std::hash::keccak256::hash_u8_to_bits(0u8);
            // ANCHOR_END: std_hash_keccak
        }
    }

    fn commit_demo(value: u64, randomizer: scalar) {
        // ANCHOR: std_commit_bhp
        // `randomizer` must be uniformly sampled and kept secret to preserve
        // the commitment's hiding property.
        let c1: field = std::commit::bhp256::commit_u64_to_field(value, randomizer);
        let c2: group = std::commit::bhp256::commit_u64_to_group(value, randomizer);
        // ANCHOR_END: std_commit_bhp
    }

    fn sig_demo(
        sig_value: signature,
        signer: address,
        message: field,
        ecdsa_sig: [u8; 65],
        vk: [u8; 33],
        prehash: [u8; 32],
    ) -> Final {
        return final {
            {
                // ANCHOR: std_sig_schnorr
                // Schnorr verification. `message` must be a `field`; hash
                // other values into a field before signing or verifying.
                let valid: bool = std::sig::verify_schnorr(sig_value, signer, message);
                // ANCHOR_END: std_sig_schnorr
            }
            {
                // ANCHOR: std_sig_ecdsa
                // ECDSA-over-prehash. `prehash` is the 32-byte digest the signer
                // hashed before signing; the wrapper does no hashing of its own.
                let valid: bool = std::sig::verify_ecdsa_digest(ecdsa_sig, vk, prehash);
                // ANCHOR_END: std_sig_ecdsa
            }
        };
    }

    fn rand_demo() -> Final {
        return final {
            // ANCHOR: std_rand
            // ChaCha randomness is deterministic per block: usable for
            // jittering rewards or sampling, but observable by the block proposer.
            let coin: bool = std::rand::chacha_bool();
            let token_id: u128 = std::rand::chacha_u128();
            // ANCHOR_END: std_rand
        };
    }

    fn serialize_demo(x: u64) {
        // ANCHOR: std_serialize
        // The tagged encoding (`to_bits`) is 26 bits wider than the raw form
        // and includes a type discriminator. `from_bits` rejects mismatched
        // tags, making round-trips type-safe.
        let bits: [bool; 90] = std::serialize::to_bits_u64(x);
        let restored: u64 = std::serialize::from_bits_u64(bits);
        // ANCHOR_END: std_serialize
    }

    fn grp_demo() {
        // ANCHOR: std_grp
        let g: group = std::grp::generator();
        let h: group = std::grp::aleo_generator();
        let x: field = std::grp::to_x_coordinate(g);
        // ANCHOR_END: std_grp
    }

    fn ctx_offchain_demo() {
        // ANCHOR: std_ctx_offchain
        // Off-chain context: who is calling, who signed, who am I.
        let me: address = std::ctx::addr();
        let invoker: address = std::ctx::caller();
        let originator: address = std::ctx::signer();
        // ANCHOR_END: std_ctx_offchain
    }

    fn ctx_onchain_demo() -> Final {
        return final {
            // ANCHOR: std_ctx_onchain
            // On-chain context; these are only accessible inside `final fn`
            // bodies or `final { ... }` async blocks.
            let height: u32 = std::ctx::block_height();
            let when: i64 = std::ctx::block_timestamp();
            let net: u16 = std::ctx::network_id();
            let owner: address = std::ctx::program_owner();
            // ANCHOR_END: std_ctx_onchain
        };
    }

    @noupgrade
    constructor() {}
}
