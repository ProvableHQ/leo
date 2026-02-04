struct Bar::[N: u32] {
    arr: [u32; N],
}

program child.aleo {
    fn main(b: Bar::[5]) -> Bar::[3] {
        let c = Bar::[4] {
            arr: [b.arr[0]; 4],
        };
        return Bar::[3] {
            arr: [c.arr[1]; 3],
        };
    }
}

// --- Next Program --- //

import child.aleo;
program parent.aleo {
    fn main() {
    }
}
