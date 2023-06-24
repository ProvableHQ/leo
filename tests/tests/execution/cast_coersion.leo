/*
namespace: Execute
expectation: Pass
cases:
    main:
    - input: ["true", "0group", "aleo1x2mh70hxp6hejyrcuhd9te5ddwvwx6papw4tfpdxp00e4sf0ps9qcu8qvn"]
    - input: ["false", "0group", "aleo1x2mh70hxp6hejyrcuhd9te5ddwvwx6papw4tfpdxp00e4sf0ps9qcu8qvn"]
    - input: ["true", "2group", "aleo1yrmttyqs4gtm8t6dtcg2vd2mtr8p6ukmpe42cp2zm0989rmtr58q0asawh"]
    - input: ["false", "2group", "aleo1yrmttyqs4gtm8t6dtcg2vd2mtr8p6ukmpe42cp2zm0989rmtr58q0asawh"]
*/

program test.aleo {

    struct foo {
        data: field,
    }

    transition main(f: bool, a: group, b: address) -> (foo, foo) {
        let first: foo = foo { data: a as field };
        if f {
            let second: foo = foo { data: 1field };
            return (first, second);
        } else {
            let second: foo = foo { data: b as field };
            return (first, second);
        }
    }
}
