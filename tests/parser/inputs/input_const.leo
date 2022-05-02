/*
namespace: Input
expectation: Pass
*/

[main]
const a: bool  = true;
const b: u8    = 2;
const c: field = 0;
const d: group = (0, 1)group;
const e: address = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8;
const f: i8    = -2;

[registers]
r0: bool  = true;
r1: u8    = 2;
r2: field = 0;
r3: group = (0, 1)group;
r4: address = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8;
r4: i8 = -1;
