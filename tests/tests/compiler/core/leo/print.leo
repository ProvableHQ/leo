/*
namespace = "Compile"
expectation = "Pass"
*/

program test.aleo {
    transition main(a: u32, b: u32) -> u32 {
        let result: u32 = a + b;
        Leo::print(result);
        return result;
    }
}
