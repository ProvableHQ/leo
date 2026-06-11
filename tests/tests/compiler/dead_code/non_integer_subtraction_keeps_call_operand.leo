/*
namespace = "Compile"
expectation = "Pass"
*/

@no_inline
fn field_identity(x: field) -> field {
    return x;
}

program test.aleo {
    fn main(x: field) -> field {
        // The subtraction result is unused, but DCE must not drop the call operand.
        let checked: field = field_identity(x) - 1field;
        return x;
    }
}
