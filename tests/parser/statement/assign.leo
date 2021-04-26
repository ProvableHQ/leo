/*
namespace: ParseStatement
expectation: Pass
*/

x = expr;

x = ();

x = x+y;

x = (x,y);

x = x();


x[0] = y;

x[0u32] = y;

x.0 = y;

x[1..2] = y;

x[..2] = y;

x[2..] = y;

x[..] = y;

x.0[0][..] = y;
