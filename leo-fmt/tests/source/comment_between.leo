program test.aleo{
}
        fn process(  a  : u64  )  ->  u64  {
                        // validate input
        assert(  a  >  0u64  );
    // compute result
    let x  :u64=a  *  2u64;
                    // apply offset
            let y:   u64  =  x  +  1u64;
      // return
    return y;
        }
