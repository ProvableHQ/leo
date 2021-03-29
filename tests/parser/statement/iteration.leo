/*
ns: ParseStatement
expectation: Pass
*/

for x in 0..7 {}

for x in 0..7 {
    return ();
}

for x in 0..99u8 {
    return ();
}

for x in 0..Self {
    return ();
}