// Conditionally add two u32 integers and log the result to the console.
function main(a: u32, b: u32) {
    if a == b {
        console.log("{}=={}",a,b); // This line should not fail.
    }
}