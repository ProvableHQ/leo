program test.aleo {
    fn t1() {
        let f1 = final {};
        let f2 = foo();
    }

    fn t2() {
        let f1 = foo();
        let f2 = final {};
    }

    fn t3() {
        let f1 = foo();
        let f2 = foo();
    }

    fn t4() {
        let f1 = final {};
        let f2 = final {};
    }
}

final fn foo() {}
