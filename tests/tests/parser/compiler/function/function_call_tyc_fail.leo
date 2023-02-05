/*
namespace: Compile
expectation: Fail
*/

program test.aleo {

    function f1(a: u8) -> u8 {
        let b: u8 = a + 1u8;
        return b;
    }

    function f3(u2: u8, u3: i16) -> u8 {
        return 0u8;
    }

    transition main(id_type: i8, s: u8) -> u8 {
        let x: i8 = 1i8;
        x = f1(1u8);

        let y: i8 = 1i8;
        let z: i16 = 100i16;
        y = f3(y, z);
        return s;
    }
}
