/*
namespace: ParseStatement
expectation: Fail
*/

let mut x = expr;

let mut x = ();

let mut x = x+y;

let mut x = (x,y);

let mut x = x();


constant mut x = expr;

constant mut x = ();

constant mut x = x+y;

constant mut x = (x,y);

constant mut x = x();


let mut x: u32 = expr;

let mut x: u32 = ();

let mut x: u32 = x+y;

let mut x: u32 = (x,y);

let mut x: u32 = x();


constant mut x: u32 = expr;

constant mut x: u32 = ();

constant mut x: u32 = x+y;

constant mut x: u32 = (x,y);

constant mut x: u32 = x();


let (x,y,,) = ();

let (,x,y) = ();

let (x,,y) = ();

let x: [u8; (2,,)] = [[0,0], [0,0]];


let x: constant = expr;

constant x: let = expr;

let

let x

let x:

let x = (a, y]);

let = 1u8;

let;

let x 1u8;

let x: u8;

let x: u8

let x: = 1;

let x: [u8] = 1;

let x: [u8;

let x: [u8; 1u8] = [1,

let dbg: u8 = ];

let 🦀: u8 = 0;

let (x) = ...;

let (x,) = ...;

let _1: u8 = 1u8;

let 1___091: u8 = 12u8;