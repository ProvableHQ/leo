struct Bar::[N: u32] {
    x: u32,
}

program child.aleo {
    fn main(b: Bar::[5]) {}
}

// --- Next Program --- //

import child.aleo;
program parent.aleo {
    fn main(c: child.aleo/Bar::[4]) {
        let b = child.aleo/Bar::[3] {
            x: 0,
        };
    }
}
