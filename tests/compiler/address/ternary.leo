/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/address1.in
 - inputs/address2.in
*/

function main(x: address) -> bool {
    const sender = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9;
    const receiver = aleo18qgam03qe483tdrcc3fkqwpp38ehff4a2xma6lu7hams6lfpgcpqy3sr3p;

    return x == sender ? receiver == x : sender == x;
}