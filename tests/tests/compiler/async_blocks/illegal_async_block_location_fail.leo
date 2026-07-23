program test.aleo {
    fn t1() {
        let f1 = final {};
    }

    fn t2() {
        for j in 0u32..2u32 {
            let f = final {};
        }
    }

    fn t3() {
        if true {
            let f = final {};
        }
    }

    @noupgrade
    constructor() {}
}

export fn f2(x: u32) {
    let f = final {};
}

export final fn f3() {
    let f = final {};
}

export fn i() {
    let f = final {};
}
