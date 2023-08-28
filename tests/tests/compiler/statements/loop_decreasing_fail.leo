/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    transition main(x: u32) -> bool {

        let count: i8 = 0i8;
        for i: i8 in 10i8..5i8 {
            count += 1i8;
        }

        return true;
    }
}