/*
namespace = "Compile"
expectation = "Pass"
*/

program test.aleo {    
    transition main(a: field) -> field {
        if a == 0field {
            return 0field;
        } else {
            return 123field / a;
        }
    }

    const C: field = 1field;

    transition main2() -> field {
        if C == 0field {
            return 0field;
        } else {
            return 123field / C;
        }
    }
}
