/*
namespace: Compile
expectation: Fail
*/

program test.aleo {

    mapping one: field => field;
    mapping one: field => field;


    transition foo() -> u8 {
        return 1u8 + 1u8;
    }
}
