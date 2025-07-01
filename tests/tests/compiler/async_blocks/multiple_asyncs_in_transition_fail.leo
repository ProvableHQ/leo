program test.aleo {
    async transition t1() {
        let f1 = async {};
        let f2 = foo();
    }

    async transition t2() {
        let f1 = foo();
        let f2 = async {};
    }

    async transition t3() {
        let f1 = foo();
        let f2 = foo();
    }

    async transition t4() {
        let f1 = async {};
        let f2 = async {};
    }

    async function foo() {}
}
