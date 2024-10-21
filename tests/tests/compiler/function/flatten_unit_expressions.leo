/*
namespace = "Compile"
expectation = "Pass"

[[configs]]
dce_enabled = true

[[configs]]
dce_enabled = false
*/

program test.aleo {

    transition bar() {
        if (true) {
            return;
        }
        return;
    }
}
