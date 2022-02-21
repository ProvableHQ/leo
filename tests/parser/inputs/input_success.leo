/*
namespace: Input
expectation: Pass
*/

[main]
a: bool  = true;                 
b: u8    = 2;                    
c: field = 0;                    
d: group = (0, 1)group;          
e: address = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8;
f: [u8; 32] = [0; 32];           
g: [[u8; 2]; 3] = [[0; 2]; 3]; 
h: (bool, bool) = (true, false); 

[registers]
r0: bool  = true;                 
r1: u8    = 2;                    
r2: field = 0;                    
r3: group = (0, 1)group;          
r4: address = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8;
r5: [u8; 32] = [0; 32];           
r6: [[u8; 2]; 3] = [[0; 2]; 3]; 
r7: (bool, bool) = (true, false); 

[constants]
c0: bool  = true;                 
c1: u8    = 2;                    
c2: field = 0;                    
c3: group = (0, 1)group;          
c4: address = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8;
c5: [u8; 32] = [0; 32];           
c6: [[u8; 2]; 3] = [[0; 2]; 3]; 
c7: (bool, bool) = (true, false); 
