program test.aleo   {
    transition compute(   a  :  u64  )  ->  u64    {
  let x  :  u64  =  a  *  2u64  ;
          return x  ;
                // end of function
        }

transition empty_with_comment(  )  {
                            // only a comment here
}
}
