/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main(x: address) -> bool {
        let sender: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
    
        return x <= sender;
    }
}
