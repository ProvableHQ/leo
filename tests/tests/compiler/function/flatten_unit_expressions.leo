/*
namespace: Compile
expectation: Pass
configs:
    - dce_enabled : true
    - dce_enabled : false
*/

program test.aleo {

    transition bar() {
        if (true) {
            return;
        }
        return;
    }
}
