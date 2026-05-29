/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "update"
input = [
    "true",
    "7u32",
    "99u32",
    "1field",
    "42field",
    "aleo1l7ytv5jqjzpxtjqttl5z9mle8ujcpac9t6tkge5f4haah4pxas8sagzecd",
    "aleo1dtpkpg3d653mdlzh6g028937qdgujecn5gw5tzh7ftcvyz7jxvfqw6t8p6",
]
[case]
program = "test.aleo"
function = "update"
input = [
    "false",
    "7u32",
    "99u32",
    "1field",
    "42field",
    "aleo1l7ytv5jqjzpxtjqttl5z9mle8ujcpac9t6tkge5f4haah4pxas8sagzecd",
    "aleo1dtpkpg3d653mdlzh6g028937qdgujecn5gw5tzh7ftcvyz7jxvfqw6t8p6",
]
*/

program test.aleo {
    mapping u32_map: u32 => u32;
    mapping field_map: u32 => field;
    mapping addr_map: u32 => address;

    fn update(
        s: bool,
        a_u32: u32,
        b_u32: u32,
        a_field: field,
        b_field: field,
        a_addr: address,
        b_addr: address,
    ) -> Final {
        return final {
            let chosen_u32: u32 = s ? a_u32 : b_u32;
            let chosen_field: field = s ? a_field : b_field;
            let chosen_addr: address = s ? a_addr : b_addr;
            u32_map.set(0u32, chosen_u32);
            field_map.set(0u32, chosen_field);
            addr_map.set(0u32, chosen_addr);
        };
    }

    @noupgrade
    constructor() {}
}
