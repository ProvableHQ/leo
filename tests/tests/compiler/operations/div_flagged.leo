/*
namespace = "Compile"
expectation = "Pass"
*/

program test.aleo {    
    transition main(a: field) -> (field, bool) {
        let (x, y): (field, bool) = a.div_flagged(0field);
        return (x, y);
    }
}
