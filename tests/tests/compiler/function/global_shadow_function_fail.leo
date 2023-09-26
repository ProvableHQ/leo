/*
namespace: Compile
expectation: Fail
*/

program test.aleo {

    function f1(a: u8) -> u8 {
        let b: u8 = a + 1u8;
        return b;
    }

    function f1(a: u8) -> u8 {
        return a * 100u8;
    }

    transition main(id_type: i8, s: u8) -> u8 {
        let x: u8 = 1u8;
        x = f1(1u8);

        let y: u8 = 1u8;
        y = f1(y);
        return s;
    }
}
