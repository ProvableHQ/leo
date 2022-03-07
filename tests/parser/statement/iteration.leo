/*
namespace: ParseStatement
expectation: Pass
*/

for x: u8 in 0..7 {}

for x: i64 in 0..7 {
    return ();
}

for x: field in 0..99u8 {
    return ();
}

for x: bool in 0..Self {
    return ();
}