/*
namespace = "Compile"
expectation = "Pass"
*/

program test.aleo {
    fn main(public x: field, y: field, a: u8, b: u8) -> field {
        // This line is dead and should be eliminated by DCE.
        let z: field = BHP256::hash_to_field(x);

        // This line should not be eliminated, as it may halt.
        let w: u8 = a + b;

        // This line should be eliminated, as it will not halt.
        let u: u8 = a.sub_wrapped(b);

        // Guards on conditionals will generate an extra `not`
        // expression that should be eliminated by DCE.
        if y == 0field {
            return x;
        } else {
            return x / y;
        }
    }
}
