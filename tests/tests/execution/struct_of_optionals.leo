/*
seed = 123456789

[case]
program = "test.aleo"
function = "main"
input = []
*/

program test.aleo {
    struct Foo {
        x: u64?, // Optional field
    }

    transition main() {
        let raw: [u64; 4] = [10u64, 0u64, 30u64, 0u64];

        // Convert into Foo structs where 0 → none
        let arr: [Foo; 4] = [
            Foo { x: raw[0] == 0u64 ? none : raw[0] },
            Foo { x: raw[1] == 0u64 ? none : raw[1] },
            Foo { x: raw[2] == 0u64 ? none : raw[2] },
            Foo { x: raw[3] == 0u64 ? none : raw[3] },
        ];

        let sum: u64 = 0u64;
        for i in 0u8..4u8 {
            sum += arr[i].x.unwrap_or((i * i) as u64);
        }

        assert(sum == 50);
    }

    @noupgrade
    async constructor() {}
}
