/*
namespace = "Compile"
expectation = "Fail"
*/

program test.aleo {
    transition main() -> u8 {
        // Check for a bug in which this wasn't properly type checked.
        return;
    }
}
