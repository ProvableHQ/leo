/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    struct Extra {
        c: u8,
    }

    struct Data {
        a: u8,
        b: u8,
        c: Extra,
    }

    inline foo(a: u8, b: u8) -> (u8, u8, Data) {
        let extra: Extra = Extra { c: a };
        let data: Data = Data { a: a, b: b, c: extra };
        if (a == b) {
            return (a, b, data);
        }
        let c: u8 = a + b;
        let d: u8 = a - b;

        return (c, d, data);
    }

    transition bar(flag1: bool, flag2: bool, a: u8, b: u8) -> (u8, u8, Data) {
        let start: (u8, u8, Data) = foo(a, b);
        if flag1 {
            start = foo(start.0, start.2.c.c);
        } else {

            if flag2 {
                start = foo(start.1, start.2.b);
            } else {
                start = foo(start.2.a, start.1);
            }

        }
        return start;
    }
}
