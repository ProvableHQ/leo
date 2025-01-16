/*
namespace = "Compile"
expectation = "Fail"
*/

program test.aleo {
    const A: i32 = 1i32 / 0i32;

    const B: u8 = 255u8 + 1u8;

    const C: field = 0field.inv();

    transition main() -> i32 {
        return 0i32;
    }
}
