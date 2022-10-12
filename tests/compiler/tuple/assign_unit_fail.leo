/*
namespace: Compile
expectation: Fail
*/

// TODO: Compilation should pass, but warnings should be emitted.
program test.aleo {

    transition foo(a: u8) -> u8 {
        let b: () = ();
        return a + a;
    }

    transition baz(a: u8) -> u8 {
        let b: () = bar();
        return a + a;
    }

    transition bar(a: u8) -> () {}
}

