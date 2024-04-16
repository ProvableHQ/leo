/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    async transition foo(public a: u8) -> (u8, Future) {
        async finalize_foo(a);
        return a + a;
    }

    async function finalize_foo(private a: u8) -> u8 {
        return a * a;
    }

    async transition bar(public a: u8) -> (u8, Future) {
        async finalize_bar(a);
        return a + a;
    }

    async function finalize_bar(a: u8) -> private u8 {
        return a * a;
    }
}
