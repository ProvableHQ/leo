function main() {
    let mut x = 4u32;
    
    {
        x = 5u32;
    }

    console.assert(x == 5u32);
}