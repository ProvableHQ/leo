/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    struct One {
        two: Two,
    }

    struct Two {
        three: Three,
        four: Four,
    }

    struct Three {
        one: One,
    }

    struct Four {
        one: One,
    }
}
