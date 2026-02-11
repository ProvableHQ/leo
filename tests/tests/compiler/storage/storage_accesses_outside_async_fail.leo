program test.aleo {
    storage x: u32;

    storage vec: [u32];

    fn foo() {
        x = 1;
        let y = x;
        vec.push(1);
        let p = vec.pop();
        let s = vec.swap_remove(0);
        let l = vec.len();
        let g = vec.get(0);
        vec.set(0, 0);
    }
}
