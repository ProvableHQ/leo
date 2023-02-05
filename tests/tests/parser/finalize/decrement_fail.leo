/*
namespace: ParseStatement
expectation: Fail
*/

decrement[foo, bar, baz];

decrement(foo, bar);

decrement();

decrement(floo)

decrement foo[bar] by baz;
