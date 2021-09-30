// Function input are mutable by default.
function main(a: bool) {
    a = true;

    console.assert(a == true);
}