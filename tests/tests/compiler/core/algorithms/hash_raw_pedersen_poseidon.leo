/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    struct Point {
        x: u64,
        y: u64,
    }

    // Test all Pedersen and Poseidon raw hash variants
    transition main(input: u64) -> (
        field, group, scalar, address,  // Pedersen64 raw
        field, group, scalar, u128,     // Pedersen128 raw
        field, group, scalar, address,  // Poseidon2 raw
        field, group, scalar, u64       // Poseidon4 & Poseidon8 raw
    ) {
        let point: Point = Point { x: 42u64, y: 84u64 };

        // Pedersen64 raw hashes
        let ped64_field_raw: field = Pedersen64::hash_to_field_raw(input);
        let ped64_group_raw: group = Pedersen64::hash_to_group_raw(input);
        let ped64_scalar_raw: scalar = Pedersen64::hash_to_scalar_raw(input);
        let ped64_addr_raw: address = Pedersen64::hash_to_address_raw(point);

        // Pedersen128 raw hashes
        let ped128_field_raw: field = Pedersen128::hash_to_field_raw(point);
        let ped128_group_raw: group = Pedersen128::hash_to_group_raw(point);
        let ped128_scalar_raw: scalar = Pedersen128::hash_to_scalar_raw(point);
        let ped128_u128_raw: u128 = Pedersen128::hash_to_u128_raw(input);

        // Poseidon2 raw hashes
        let psd2_field_raw: field = Poseidon2::hash_to_field_raw(input);
        let psd2_group_raw: group = Poseidon2::hash_to_group_raw(input);
        let psd2_scalar_raw: scalar = Poseidon2::hash_to_scalar_raw(input);
        let psd2_addr_raw: address = Poseidon2::hash_to_address_raw(point);

        // Poseidon4 raw hashes
        let psd4_field_raw: field = Poseidon4::hash_to_field_raw(point);
        let psd4_group_raw: group = Poseidon4::hash_to_group_raw(point);
        let psd4_scalar_raw: scalar = Poseidon4::hash_to_scalar_raw(point);

        // Poseidon8 raw hashes
        let psd8_u64_raw: u64 = Poseidon8::hash_to_u64_raw(point);

        return (
            ped64_field_raw, ped64_group_raw, ped64_scalar_raw, ped64_addr_raw,
            ped128_field_raw, ped128_group_raw, ped128_scalar_raw, ped128_u128_raw,
            psd2_field_raw, psd2_group_raw, psd2_scalar_raw, psd2_addr_raw,
            psd4_field_raw, psd4_group_raw, psd4_scalar_raw, psd8_u64_raw
        );
    }
}
