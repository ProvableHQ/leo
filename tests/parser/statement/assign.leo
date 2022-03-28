/*
namespace: ParseStatement
expectation: Pass
*/

x = expr;

x = ();

x = x+y;

x = (x,y);

x = x();

x += expr;

x += ();

x += x+y;

x += x();

x -= expr;

x -= ();

x -= x+y;

x -= x();

x *= expr;

x *= ();

x *= x+y;

x *= x();

x.0 *= y;


x /= expr;

x /= ();

x /= x+y;

x /= x();

x.0 /= y;

x **= expr;

x **= ();

x **= x+y;

x **= x();



x.0 **= y;
