program child1.aleo {
    record R {owner: address}
    fn main(r: R) { 
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import child1.aleo;
program parent.aleo {

    fn main(addr: address) {
        let r = child1.aleo/R { owner: addr };
    }
}
