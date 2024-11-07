/*
namespace = "Compile"
expectation = "Fail"
*/

program test.aleo {
    mapping m: u8 => u8;

    transition main(id_type: i8, s: u8) -> u8 {
        let m: u8 = 1u8;
        return m;
    }
}
