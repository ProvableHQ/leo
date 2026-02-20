/*
seed = 123456789
min_height = 16

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

struct S {
    x: field?,
    y: group?,
}

struct Outer {
    inner: S?,
    arr: [field?; 3]?,
}

program test_hash_and_commit.aleo {
    fn hash(x: field, y: group) -> (public [field; 6], public group) {
        // Wrap x and y into optionals, include a none.
        let sx: field? = x;
        let sy: group? = none;

        let s: S? = S {
            x: sx,
            y: sy,
        };

        // Outer struct holding the optional s and an optional array
        let outer: Outer? = Outer {
            inner: s,
            arr: [1field, none, 3field],
        };

        // Use the outer optional in hashing
        return ([
            BHP256::hash_to_field(outer),
            BHP512::hash_to_field(outer),
            BHP768::hash_to_field(outer),
            BHP1024::hash_to_field(outer),
            Keccak256::hash_to_field(outer),
            Keccak384::hash_to_field(outer),
        ],
            Keccak256::hash_to_group([1u8, 2u8, 3u8, 4u8, 5u8, 6u8])
        );
    }

    fn commit(x: field, y: group, z: scalar) -> public [field; 6] {
        let sx: field? = none;
        let sy: group? = y;

        let s: S? = S {
            x: sx,
            y: sy,
        };

        let outer: Outer? = Outer {
            inner: s,
            arr: [10field, 20field, none],
        };

        return [
            BHP256::commit_to_field(outer, z),
            BHP512::commit_to_field(outer, z),
            BHP768::commit_to_field(outer, z),
            BHP1024::commit_to_field(outer, z),
            Pedersen64::commit_to_field(12u8, z),
            Pedersen128::commit_to_field(12u8, z),
        ];
    }

    @noupgrade
    constructor() {}
}
