/*
namespace: Compile
expectation: Pass
*/

function one(x: bool) -> bool {
    return x && true;
}

function main(y: bool) -> bool {
    let a: bool = one(y) && one(y);

    return (a == true) == y;
}
