program test.aleo {
    async fn t1() {
        let f1 = async {};
        let f2 = foo();
    }

    async fn t2() {
        let f1 = foo();
        let f2 = async {};
    }

    async fn t3() {
        let f1 = foo();
        let f2 = foo();
    }

    async fn t4() {
        let f1 = async {};
        let f2 = async {};
    }
}

async fn foo() {}
