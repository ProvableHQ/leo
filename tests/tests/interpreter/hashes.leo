/*
seed = 123456789

[case]
program = "test_hash_and_commit.aleo"
function = "hash"
input = ["123456field", "0group"]
[case]
program = "test_hash_and_commit.aleo"
function = "hash"
input = ["987654321field", "0group"]
[case]
program = "test_hash_and_commit.aleo"
function = "commit"
input = ["123456field", "0group", "1scalar"]
[case]
program = "test_hash_and_commit.aleo"
function = "commit"
input = ["987654321field", "0group", "1scalar"]
*/


program test_hash_and_commit.aleo {
    struct S {
        x: field,
        y: group,
    }

    transition hash(x: field, y: group) -> ([field; 12], group) {
        let s = S {
            x,
            y,
        };

        // If we include all 14 hash functions the program is too large.
        return ([
            BHP256::hash_to_field(s),
            BHP512::hash_to_field(s),
            BHP768::hash_to_field(s),
            BHP1024::hash_to_field(s),
            Keccak256::hash_to_field(s),
            Keccak384::hash_to_field(s),
            Pedersen64::hash_to_field(12u8),
            Poseidon2::hash_to_field(s),
            Poseidon4::hash_to_field(s),
            Poseidon8::hash_to_field(s),
            SHA3_256::hash_to_field(s),
            SHA3_512::hash_to_field(s),
        ],
            Keccak256::hash_to_group([1u8, 2u8, 3u8, 4u8, 5u8, 6u8])
        );
    }

    transition commit(x: field, y: group, z: scalar) -> [field; 6] {
        let s = S {
            x,
            y,
        };

        return [
            BHP256::commit_to_field(s, z),
            BHP512::commit_to_field(s, z),
            BHP768::commit_to_field(s, z),
            BHP1024::commit_to_field(s, z),
            Pedersen64::commit_to_field(12u8, z),
            Pedersen128::commit_to_field(12u8, z),
        ];
    }

    @noupgrade
    async constructor() {}
}
