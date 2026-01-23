program test.aleo {
    fn t1() {
        let f1 = async {};
        let f2 = foo();
    }

    fn t2() {
        let f1 = foo();
        let f2 = async {};
    }

    fn t3() {
        let f1 = foo();
        let f2 = foo();
    }

    fn t4() {
        let f1 = async {};
        let f2 = async {};
    }
}

onchain fn foo() {}
