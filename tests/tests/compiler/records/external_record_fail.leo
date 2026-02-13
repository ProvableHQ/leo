program child1.aleo {
    record R {owner: address}
    transition main(r: R) { 
    }

    @noupgrade
    async constructor() {}
}

// --- Next Program --- //

import child1.aleo;
program parent.aleo {

    transition main(addr: address) {
        let r = child1.aleo/R { owner: addr };
    }
}
