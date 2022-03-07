/*
namespace: ParseStatement
expectation: Pass
*/

let x: u8 = expr;

let x: () = ();

let x: u16 = x+y;

let x: (u32, u64) = (x,y);

let x: u8 = x();


const x: i8 = expr;

const x: () = ();

const x: i16 = x+y;

const x: (i32, i64) = (x,y);

const x: i8 = x();


let x: u32 = expr;

let x: u32 = ();

let x: u32 = x+y;

let x: u32 = (x,y);

let x: u32 = x();


const x: u32 = expr;

const x: u32 = ();

const x: u32 = x+y;

const x: u32 = (x,y);

const x: u32 = x();






let (x, y): (field, group) = expr;

let (x, y): (u8, u16) = ();

let (x, y): (i8, i16) = x+y;

let (x, y): (u32, u64) = (x,y);

let (x, y): (i32, i64) = x();


const (x, y): (field, group)  = expr;

const (x, y): (u8, u16) = ();

const (x, y): (i8, i16) = x+y;

const (x, y): (u32, u64) = (x,y);

const (x, y): (i32, i64) = x();


let (x, y): u32 = expr;

let (x, y): u32 = ();

let (x, y): u32 = x+y;

let (x, y): u32 = (x,y);

let (x, y): u32 = x();


const (x, y): u32 = expr;

const (x, y): u32 = ();

const (x, y): u32 = x+y;

const (x, y): u32 = (x,y);

const (x, y): u32 = x();


let (x,y,): () = ();

let (x,): u8 = ();


let x: [char; _] = "Hello, World!";

let x: [[u8; 2]; 2] = [[0,0], [0,0]];
