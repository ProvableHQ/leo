/*
namespace: ParseStatement
expectation: Pass
*/

x = expr;

x = ();

x = x+y;

x = (x,y);

x = x();

x.y = y;

x += expr;

x += ();

x += x+y;

x += x();

x.0 += y;

x.y += y;

x -= expr;

x -= ();

x -= x+y;

x -= x();

x.y -= y;

x *= expr;

x *= ();

x *= x+y;

x *= x();

x.0 *= y;

x.y *= y;


x /= expr;

x /= ();

x /= x+y;

x /= x();

x.0 /= y;

x.y /= y;


x **= expr;

x **= ();

x **= x+y;

x **= x();



x.0 **= y;

x.y **= y;
