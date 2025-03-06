/*
namespace = "Compile"
expectation = "Pass"
*/

program test.aleo {    
    transition main() -> (field, bool) {
        let (x, y): (field, bool) = 0field.inv_flagged(); 
        return (x, y);
    }
}
