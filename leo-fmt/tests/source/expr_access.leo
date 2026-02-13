program test.aleo{
struct Point  {  x:u64,
    y  :  u64  }

	function main(  )  ->  u64   {

                let arr  :[u64;3]  =  [1u64  ,  2u64  ,  3u64];
    let first  :u64  =  arr  [  0u32  ]  ;

let tup
    :(u64,u64)  =(  1u64  ,  2u64  );

	    let a:u64  =tup.0;
            let b:u64
                =tup.1;

let p:  Point
    =Point  {  x  :  1u64  ,  y  :  2u64  };

	        let px :u64  =  p.x;
        return first  ;

    }
}
