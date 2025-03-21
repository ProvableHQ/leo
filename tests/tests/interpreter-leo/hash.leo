
program test.aleo {
    transition main() -> field {
        let a: field = 1234567890field;
        return BHP256::hash_to_field(a)
            * SHA3_256::hash_to_field(a)
            * Poseidon2::hash_to_field(a)
            * Poseidon4::hash_to_field(a)
            * Poseidon8::hash_to_field(a);
    }
}
