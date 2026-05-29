/*
seed = 123456789

[case]
program = "test.aleo"
function = "select"
input = [
    "true",
    "7u32",
    "99u32",
    "true",
    "false",
    "1field",
    "42field",
    "aleo1l7ytv5jqjzpxtjqttl5z9mle8ujcpac9t6tkge5f4haah4pxas8sagzecd",
    "aleo1dtpkpg3d653mdlzh6g028937qdgujecn5gw5tzh7ftcvyz7jxvfqw6t8p6",
]
[case]
program = "test.aleo"
function = "select"
input = [
    "false",
    "7u32",
    "99u32",
    "true",
    "false",
    "1field",
    "42field",
    "aleo1l7ytv5jqjzpxtjqttl5z9mle8ujcpac9t6tkge5f4haah4pxas8sagzecd",
    "aleo1dtpkpg3d653mdlzh6g028937qdgujecn5gw5tzh7ftcvyz7jxvfqw6t8p6",
]
*/

program test.aleo {
    fn select(
        s: bool,
        a_u32: u32,
        b_u32: u32,
        a_bool: bool,
        b_bool: bool,
        a_field: field,
        b_field: field,
        a_addr: address,
        b_addr: address,
    ) -> (u32, bool, field, address) {
        let p: u32 = s ? a_u32 : b_u32;
        let q: bool = s ? a_bool : b_bool;
        let r: field = s ? a_field : b_field;
        let t: address = s ? a_addr : b_addr;
        return (p, q, r, t);
    }

    @noupgrade
    constructor() {}
}
