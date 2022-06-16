/*
namespace: Compile
expectation: Pass
input_file: inputs/address1.in
*/


function main (x: address) -> bool {
    let a: address = aleo1fj982yqchhy973kz7e9jk6er7t6qd6jm9anplnlprem507w6lv9spwvfxx;
    let b = x.eq(a);
    let c = x.neq(a);
    return c;
}