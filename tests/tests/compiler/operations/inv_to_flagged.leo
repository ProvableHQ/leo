/*
namespace = "Compile"
expectation = "Pass"
*/

program test.aleo {    
    transition main(a: field) -> field {
        if a == 0field {
            return 0field;
        } else {
            return a.inv();
        }
    }

    const C: field = 1field;

    transition main2() -> field {
        if C == 0field {
            return 0field;
        } else {
            return C.inv();
        }
    }
}
