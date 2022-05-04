/*
namespace: ParseStatement
expectation: Pass
*/

for x: u8 in 0u8..7u8 {}

for x: i64 in 0i64..7i64 {
    return 1u8;
}

for x: field in 0field..99u8 {
    return 1u8;
}

for x: bool in 0u8..Self {
    return 1u8;
}