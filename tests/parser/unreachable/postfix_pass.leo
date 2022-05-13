/*
namespace: ParseStatement
expectation: Pass

# These ones do not hit the unreachable as they are treated as valid idents rather than postfix.
*/

let x: u8 = aimport;

let x: u8 = a_;

let x: u8 = aas;

let x: u8 = aconsole;

let x: u8 = aconst;

let x: u8 = alet;

let x: u8 = afor;

let x: u8 = aif;

let x: u8 = aelse;

let x: u8 = ai8;

let x: u8 = ai16;

let x: u8 = ai32;

let x: u8 = ai64;

let x: u8 = ai128;

let x: u8 = au8;

let x: u8 = au16;

let x: u8 = au32;

let x: u8 = au64;

let x: u8 = au128;

let x: u8 = areturn;

let x: u8 = aself;

let x: u8 = aSelf;

let x: u8 = atrue;

let x: u8 = afalse;

let x: u8 = a0;
