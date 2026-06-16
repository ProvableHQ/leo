program child.aleo {
    record child_rec {
        owner: address,
    }

    fn main(t: child_rec) {}

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import child.aleo;

program inter.aleo {
    fn use_child(c: child.aleo::child_rec) {}

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import inter.aleo;

program parent.aleo {
    fn main() {}

    @noupgrade
    constructor() {}
}
