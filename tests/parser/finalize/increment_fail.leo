/*
namespace: ParseStatement
expectation: Fail
*/

increment[foo, bar, baz];

increment(foo, bar);

increment();

increment(floo)

increment foo[bar] by baz;
