program test.aleo {
    fn t1() {
        let f1 = async {};
    }

    async fn t2() {
        for j in 0u32..2u32 {
            let f = async {};
        }
    }

    async fn t3() {
        if true {
            let f = async {};
        }
    }
}

fn f2(x: u32) {
    let f = async {};
}

async fn f3() {
    let f = async {};
}

fn i() {
    let f = async {};
}
