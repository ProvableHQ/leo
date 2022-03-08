/*
namespace: ParseStatement
expectation: Pass
*/

x = expr;

x = ();

x = x+y;

x = (x,y);

x = x();


x[0u8] = y;

x[0u32] = y;

x.0 = y;

x[1u16..2u32] = y;

x[..2u64] = y;

x[2u8..] = y;

x[..] = y;

x.0[0u8][..] = y;
