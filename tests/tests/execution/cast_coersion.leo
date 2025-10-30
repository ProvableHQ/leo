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

    @noupgrade
    async constructor() {}
}

// --- Test --- //

import test.aleo;
program test_test.aleo {

    @test
    transition main_1() {
        // f=true, a=0group, b=aleo1x2mh70hxp6hejyrcuhd9te5ddwvwx6papw4tfpdxp00e4sf0ps9qcu8qvn
        let output = test.aleo/main(true, 0group, aleo1x2mh70hxp6hejyrcuhd9te5ddwvwx6papw4tfpdxp00e4sf0ps9qcu8qvn);
        assert_eq(foo { data: 0group as field }, output.0);
        assert_eq(foo { data: 1field }, output.1);
    }

    @test
    transition main_2() {
        // f=false, a=0group, b=aleo1x2mh70hxp6hejyrcuhd9te5ddwvwx6papw4tfpdxp00e4sf0ps9qcu8qvn
        let output = test.aleo/main(false, 0group, aleo1x2mh70hxp6hejyrcuhd9te5ddwvwx6papw4tfpdxp00e4sf0ps9qcu8qvn);
        assert_eq(foo { data: 0group as field }, output.0);
        assert_eq(foo { data: aleo1x2mh70hxp6hejyrcuhd9te5ddwvwx6papw4tfpdxp00e4sf0ps9qcu8qvn as field }, output.1);
    }

    @test
    transition main_3() {
        // f=true, a=2group, b=aleo1yrmttyqs4gtm8t6dtcg2vd2mtr8p6ukmpe42cp2zm0989rmtr58q0asawh
        let output = test.aleo/main(true, 2group, aleo1yrmttyqs4gtm8t6dtcg2vd2mtr8p6ukmpe42cp2zm0989rmtr58q0asawh);
        assert_eq(foo { data: 2group as field }, output.0);
        assert_eq(foo { data: 1field }, output.1);
    }

    @test
    transition main_4() {
        // f=false, a=2group, b=aleo1yrmttyqs4gtm8t6dtcg2vd2mtr8p6ukmpe42cp2zm0989rmtr58q0asawh
        let output = test.aleo/main(false, 2group, aleo1yrmttyqs4gtm8t6dtcg2vd2mtr8p6ukmpe42cp2zm0989rmtr58q0asawh);
        assert_eq(foo { data: 2group as field }, output.0);
        assert_eq(foo { data: aleo1yrmttyqs4gtm8t6dtcg2vd2mtr8p6ukmpe42cp2zm0989rmtr58q0asawh as field }, output.1);
    }

    @noupgrade
    async constructor() {}
}
