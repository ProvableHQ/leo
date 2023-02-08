/*
namespace: Input
expectation: Fail
*/

[main]
public constant a: bool  = true;
public constant b: u8    = 2u8;
public constant c: field = 0field;
public constant d: group = (0, 1)group;
public constant e: address = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8;
public constant f: i8    = -2i8;


[registers]
r0: bool  = true;
r1: u8    = 2u8;
r2: field = 0field;
r3: group = (0, 1)group;
r4: address = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8;
r4: i8 = -1i8;
