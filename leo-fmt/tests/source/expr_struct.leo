	struct Point  {
  x  :u64  ,
            y  :  u64  }

program test.aleo   {
    fn main(  )  ->  Point   {

let p  :Point  =  Point  {   x  :  1u64  ,
        y  :  2u64   }  ;

                let q  :  Point  =  Point  {
	    x  :  p.x  ,  y  :  p.y  };

            return p  ;

    }
}
