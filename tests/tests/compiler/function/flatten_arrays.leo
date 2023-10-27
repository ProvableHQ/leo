/*
namespace: Compile
expectation: Pass
*/

program test.aleo {

    struct Data {
        data: [u8; 2],
    }

    function foo(a: u8, b: u8) -> [Data; 2] {
        let data_1: Data = Data { data: [a, b] };
        let data_2: Data = Data { data: [b, a] };
        if (a == b) {
            return [data_1, data_2];
        }

        let data_3: Data = Data { data: [2u8 * data_1.data[0u8], 4u8 * data_2.data[1u8]] };

        return [data_2, data_3];
    }

    transition bar(flag1: bool, flag2: bool, a: u8, b: u8) -> [Data; 2] {
        let start: [Data; 2] = foo(a, b);
        if flag1 {
            start = foo(start[0u8].data[0u8], start[1u8].data[1u8]);
        } else {
            if flag2 {
                start = foo(start[1u8].data[0u8], start[0u8].data[1u8]);
            } else {
                start = foo(start[0u8].data[1u8], start[1u8].data[0u8]);
            }
        }
        return start;
    }
}
