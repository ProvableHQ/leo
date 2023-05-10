/*
namespace: Compile
expectation: Pass
*/

/*
The program should produce the following bytecode:

program test.aleo;

record dummy:
    owner as address.private;
    data as u8.private;

closure eliminate_unused_function_call:
    input r0 as u8;
    input r1 as u8;
    add r0 r1 into r2;
    output r2 as u8;

function foo:
    input r0 as u8.private;
    input r1 as u8.private;
    add r0 r1 into r2;
    output r2 as u8.private;
*/

program test.aleo {

    record dummy {
        owner: address,
        data: u8,
    }

    function eliminate_unused_function_call(a: u8, b: u8) -> u8 {
        return a + b;
    }

    inline inline_and_eliminate(a: u8, b: u8) -> u8 {
        return a * b;
    }

    transition foo(a: u8, b: u8) -> u8 {
        let c: u8 = a + b;
        let d: u8 = eliminate_unused_function_call(a, b);
        let e: u8 = 0u8;
        if (a == b) {
            e = inline_and_eliminate(a, b);
        }
        let f: dummy = dummy {
            owner: self.caller,
            data: e,
        };
        return a + b;
    }
}
