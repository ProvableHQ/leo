/*
namespace = "Compile"
expectation = "Pass"

[[configs]]
dce_enabled = true
*/

program test.aleo {

    transition bar() {
        if (true) {
            return;
        }
        return;
    }
}
