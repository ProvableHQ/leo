/*
namespace = "Compile"
expectation = "Fail"
*/

program test.aleo {    
    struct S {
        x: (),
        y: u8,
    }

    transition main(x: ()) {}

    transition other(x: [(); 2]) {}

    transition yet_another() -> [(); 2] {}

}
