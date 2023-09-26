/*
namespace: Compile
expectation: Fail
*/


program test.aleo {
    struct s1 {
        f1: u32,
        f2: u32
    }
    struct s1 {
        f1: u32,
        f2: u32,
        f3: u32
    }

    transition main(id_type: i8, s: s1) -> u32 {
        let x: u32 = s.f1;
        x = x + 1u32;
        return x;
    }
}

