/*
namespace: Compile
expectation: Fail
*/

// TODO: Compilation should pass, but warnings should be emitted.

@program
function foo(a: u8) -> u8 {
    let b: () = ();
    return a + a;
}

@program
function baz(a: u8) -> u8 {
    let b: () = bar();
    return a + a;
}

function bar(a: u8) -> () {}
