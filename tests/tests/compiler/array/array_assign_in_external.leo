// This test ensures that `WriteTransforming` works on array assignment in Leo stubs

program child.aleo {
    fn foo(b: u32) -> [u32; 3] {
        let a = [b; 3];
        a[0] = 1;
        a[1] = 2;
        a[2] = 3;
        return a;
    }
}

// --- Next Program --- //

import child.aleo;
program parent.aleo {
    fn main() { }
}
