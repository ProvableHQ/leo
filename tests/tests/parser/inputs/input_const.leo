/*
namespace: Input
expectation: Fail
*/

[main]
const a: bool  = true;
const b: u8    = 2u8;
const c: field = 0field;
const d: group = (0, 1)group;
const e: address = aleo1fj982yqchhy973kz7e9jk6er7t6qd6jm9anplnlprem507w6lv9spwvfxx;
const f: i8    = -2i8;

[registers]
r0: bool  = true;
r1: u8    = 2u8;
r2: field = 0field;
r3: group = (0, 1)group;
r4: address = aleo1fj982yqchhy973kz7e9jk6er7t6qd6jm9anplnlprem507w6lv9spwvfxx;
r4: i8 = -1i8;
