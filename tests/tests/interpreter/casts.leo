/*
seed = 123456789

[case]
program = "test_casts.aleo"
function = "do_casts"
input = ["1234field", "0group", "12scalar", "100u32", "50i32"]
[case]
program = "test_casts.aleo"
function = "do_casts"
input = ["12345field", "0group", "10scalar", "101u32", "51i32"]
*/


program test_casts.aleo {
    transition do_casts(a: field, b: group, c: scalar, d: u32, e: i32) -> (
        field,
        field,
        field,
        field,
        field,
        u16,
        u16,
        u16,
        u16,
        u16,
    ) {
        return (
            a as field,
            b as field,
            c as field,
            d as field,
            e as field,
            a as u16,
            b as u16,
            c as u16,
            d as u16,
            e as u16,
        );
    }

    @noupgrade
    async constructor() {}
}
