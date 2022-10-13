/*
namespace: Compile
expectation: Fail
*/

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

