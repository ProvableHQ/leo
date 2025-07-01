program test.aleo {
    transition t1() {
        let f1 = async {};
    }

    async transition t2() {
        for j in 0u32..2u32 {
            let f = async {};
        }
    }

    async transition t3() {
        if true {
            let f = async {};
        }
    }

    function f2(x: u32) {
        let f = async {};
    }

    async function f3() {
        let f = async {};
    }

    inline i() {
        let f = async {};
    }
}
