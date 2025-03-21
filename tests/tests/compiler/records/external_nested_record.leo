
program child.aleo {
    record child_rec {
        owner: address
    }
    transition main(t: child_rec) {}
}

// --- Next Program --- //

import child.aleo;
program inter.aleo {
    transition use_child(c: child.aleo/child_rec){}
}

// --- Next Program --- //

import inter.aleo;
program parent.aleo {
    transition main() {}
}