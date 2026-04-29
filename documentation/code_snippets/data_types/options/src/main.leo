// ANCHOR: file
// Struct of options
struct Point {
    x: u32?,
    y: u32?
}

program options_demo.aleo {
    fn demo() -> u16 {
        // Array of options
        let arr: [u16?; 2] = [1u16, none];
        let first_val = arr[0].unwrap();        // Returns 1u16
        let second_val = arr[1].unwrap_or(0u16); // Returns 0u16

        // Structs have option variant as well
        let p: Point? = none;
        let p_val = p.unwrap_or(Point { x: 0u32, y: none }); // Returns default

        return first_val + second_val;
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
