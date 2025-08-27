program test.aleo {
    transition main() {
        let x: u16? = None;
        let y = x;
        let z = y.unwrap(); 
    }
}
