/*
namespace: Compile
expectation: Fail
*/


program test.aleo {
    struct s1 {
        f1: u32,
        f2: u32
    }

    transition main(id_type: i8, s: s1) -> i8 {
        let x: i8 = s.f1;
        x = x + 1i8;
        return x;
    }
}

