/*
namespace = "Compile"
expectation = "Pass"
*/

program test.aleo {
    transition main(public x: field, y: field) -> field {
        // This line is dead and should be eliminated by DCE.
        let z: field = BHP256::hash_to_field(x);

        // Guards on conditionals will generate an extra `not`
        // expression that should be eliminated by DCE.
        if y == 0field {
            return x; 
        } else {
            return x / y;
        }
    }
}
