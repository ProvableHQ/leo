program b28681.aleo {
    fn main() {
        let arr = [1u32, 2u32, 3u32];
        let _x0 = arr[4]; // out of bounds
    }

    @noupgrade
    constructor() {}
}
