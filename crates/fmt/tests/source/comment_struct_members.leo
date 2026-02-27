program test.aleo  {
        struct Foo  {
        // leading comment on x
                    x  :  u32  ,
            // trailing comment on x
    y  :  u32  ,
                /* block comment on y */
        }

    struct Bar  {
                // comment before first
        a  :  u32  ,
            b  :  bool  ,
                        // inline comment
    }       // after struct

            fn main(  f  :  Foo  ,  b  :  Bar  )  ->  u32  {
                    return  f.x  +  b.a  ;
        }
}
