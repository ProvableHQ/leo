/*
namespace = "Compile"
expectation = "Pass"
*/

program test.aleo {
    mapping mfield: field => field;
    mapping mgroup: group => group;
    mapping mu32: u32 => u32;

    async function foo() {
        mfield.set(
            BHP512::hash_to_field(1234567u128),
            BHP512::hash_to_field(12field),
        );
        mgroup.set(
            BHP512::hash_to_group(12u32),
            BHP512::hash_to_group(123123scalar),
        );
        mu32.set(
            BHP512::hash_to_u32(98765field),
            BHP512::hash_to_u32(BHP512::hash_to_group(168field)),
        );
    }

    async transition main() -> Future {
        return foo();
    }
}
