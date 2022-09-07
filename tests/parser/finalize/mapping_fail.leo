/*
namespace: Parse
expectation: Fail
*/

mapping u8: u8 => u8;

mapping foo: bar -> baz;

mapping foo: (bar => baz);

mapping foo: [bar -> baz];

mapping foo bar => baz;
