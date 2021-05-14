/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/address1.in
 - inputs/address2.in
*/

function main(x: address) -> bool {
    const sender = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8;
    const receiver = aleo18qgam03qe483tdrcc3fkqwpp38ehff4a2xma6lu7hams6lfpgcpq3dq05r;

    return x == sender ? receiver == x : sender == x;
}