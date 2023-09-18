/*
namespace: Execute
expectation: Pass
cases:
    bar:
    - input: ["true", "true", "1u8", "0u8"]
    - input: ["true", "false", "1u8", "1u8"]
    - input: ["false", "true", "2u8", "1u8"]
    - input: ["false", "false", "2u8", "2u8"]
    blar:
    - input: ["true", "true", "1u8", "0u8"]
    - input: ["true", "false", "1u8", "1u8"]
    - input: ["false", "true", "2u8", "1u8"]
    - input: ["false", "false", "2u8", "2u8"]
*/

// In this test, we expect bar and blar to produce the same result for all inputs.

program test.aleo {
    struct Extra {
        c: u8,
    }

    struct Data {
        a: u8,
        b: u8,
        c: Extra,
    }

    inline foo(a: u8, b: u8, input: Data) -> (u8, u8, Data) {
        let extra: Extra = Extra { c: a };
        let data: Data = Data { a: a, b: b, c: extra };
        if (a == b) {
            return (a, b, data);
        }
        let c: u8 = a + b;
        let d: u8 = a - b;
        c = c + input.a;
        d = d + input.b;

        return (c, d, data);
    }

    function floo(a: u8, b: u8, input: Data) -> (u8, u8, Data) {
        let extra: Extra = Extra { c: a };
        let data: Data = Data { a: a, b: b, c: extra };
        if (a == b) {
            return (a, b, data);
        }
        let c: u8 = a + b;
        let d: u8 = a - b;
        c = c + input.a;
        d = d + input.b;

        return (c, d, data);
    }

    transition bar(flag1: bool, flag2: bool, a: u8, b: u8) -> (u8, u8, Data) {
        let data: Data = Data { a: a, b: b, c: Extra { c: a } };
        let start: (u8, u8, Data) = foo(a, b, data);
        // (2, 1, Data { a: 1, b: 0, c: Extra { c: 1 } })
        if flag1 {
            // foo (2, 1, Data { a: 1, b: 0, c: Extra { c: 1 } })
            start = foo(start.0, start.2.c.c, start.2);
            //
        } else {
            if flag2 {
                // foo (1, 0, Data { a: 1, b: 0, c: Extra { c: 1 } })
                start = foo(start.1, start.2.b, start.2);
            } else {
                // foo (1, 1, Data { a: 1, b: 0, c: Extra { c: 1 } })
                start = foo(start.2.a, start.1, start.2);
            }
        }
        return start;
    }

    transition blar(flag1: bool, flag2: bool, a: u8, b: u8) -> (u8, u8, Data) {
        let data: Data = Data { a: a, b: b, c: Extra { c: a } };

        let start: (u8, u8, Data) = floo(a, b, data);
        if flag1 {
            start = floo(start.0, start.2.c.c, start.2);
        } else {
            if flag2 {
                start = floo(start.1, start.2.b, start.2);
            } else {
                start = floo(start.2.a, start.1, start.2);
            }
        }
        return start;
    }

}
