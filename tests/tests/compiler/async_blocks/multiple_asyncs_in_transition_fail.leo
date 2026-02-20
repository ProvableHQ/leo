program test.aleo {
    fn t1() {
        let f1 = final {};
        let f2 = final { foo(); };
    }

    fn t2() {
        let f1 = final { foo(); };
        let f2 = final {};
    }

    fn t3() {
        let f1 = final { foo(); };
        let f2 = final { foo(); };
    }

    fn t4() {
        let f1 = final {};
        let f2 = final {};
    }
}

final fn foo() {}
