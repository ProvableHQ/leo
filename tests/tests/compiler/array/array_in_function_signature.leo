/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition foo(a: [bool; 8]) -> bool {
        return true;
    }

    transition bar(a: [bool; 8]) -> [bool; 8] {
        return a;
    }

    function baz(a: [bool; 8]) -> bool {
        return true;
    }

    function qux(a: [bool; 8]) -> [bool; 8] {
        return a;
    }
}
