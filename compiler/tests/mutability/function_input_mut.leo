// Adding the `mut` keyword makes a function variable mutable.
function main(mut a: bool) {
    a = true;

    console.assert(a == true);
}