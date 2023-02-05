/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: scalar, b: group, c: scalar) -> bool {
       let d: group = 1817767092074430972953743941103352519057913259183777531581123188265134806220group * a;
       let e: group = a * 1817767092074430972953743941103352519057913259183777531581123188265134806220group;
       let f: group = b * a;
       let g: group = a * b;
    
       return a * g == d;
    }}
